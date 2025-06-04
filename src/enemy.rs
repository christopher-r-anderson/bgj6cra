use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    collisions::CollisionLayer,
    energy::{AttackPoints, HitPoints},
    explosion::ExplosionChain,
};

pub const ENEMY_SIZE: Vec2 = Vec2::new(28., 28.);
pub const ENEMY_BASE_SIZE: Vec2 = Vec2::new(80., 30.);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<EnemyClass>()
            .register_type::<EnemyTeam>()
            .add_observer(on_enemy_collision)
            .add_observer(remove_enemy_when_destroyed)
            .add_observer(spawn_chain_when_destroyed_by_player)
            .add_systems(Startup, setup);
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum EnemyClass {
    Enemy,
    EnemyBase,
    Land,
    Projectile,
}

impl std::fmt::Display for EnemyClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnemyClass::Enemy => write!(f, "Enemy"),
            EnemyClass::EnemyBase => write!(f, "EnemyBase"),
            EnemyClass::Land => write!(f, "Land"),
            EnemyClass::Projectile => write!(f, "Projectile"),
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum EnemyTeam {
    Alien,
    Demon,
    Ghost,
    Skull,
}

impl std::fmt::Display for EnemyTeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnemyTeam::Alien => write!(f, "Alien"),
            EnemyTeam::Demon => write!(f, "Demon"),
            EnemyTeam::Ghost => write!(f, "Ghost"),
            EnemyTeam::Skull => write!(f, "Skull"),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(EnemyBundle::new(
        &asset_server,
        EnemyTeam::Alien,
        EnemyClass::EnemyBase,
        vec2(0., 330.),
    ));
    commands.spawn(EnemyBundle::new(
        &asset_server,
        EnemyTeam::Alien,
        EnemyClass::Enemy,
        vec2(-60., 200.),
    ));
    commands.spawn(EnemyBundle::new(
        &asset_server,
        EnemyTeam::Alien,
        EnemyClass::Enemy,
        vec2(-20., 200.),
    ));
    commands.spawn(EnemyBundle::new(
        &asset_server,
        EnemyTeam::Alien,
        EnemyClass::Enemy,
        vec2(40., 0.),
    ));
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    class: EnemyClass,
    team: EnemyTeam,
    name: Name,
    scene: SceneRoot,
    hp: HitPoints,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    collision_events_enabled: CollisionEventsEnabled,
    collision_layers: CollisionLayers,
}

impl EnemyBundle {
    pub fn new(
        asset_server: &AssetServer,
        team: EnemyTeam,
        class: EnemyClass,
        position: Vec2,
    ) -> Self {
        match class {
            EnemyClass::EnemyBase => Self {
                enemy: Enemy,
                name: Name::new(format!("{team} {class}")),
                team,
                class,
                scene: SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one-base.glb")),
                ),
                hp: HitPoints(3),
                transform: Transform::from_xyz(position.x, position.y, 2.),
                rigid_body: RigidBody::Static,
                collider: Collider::rectangle(ENEMY_BASE_SIZE.x, ENEMY_BASE_SIZE.y),
                collision_events_enabled: CollisionEventsEnabled,
                collision_layers: CollisionLayers::new(
                    CollisionLayer::EnemyBase,
                    [CollisionLayer::PlayerProjectile],
                ),
            },

            /* TODO: use this for EnemyClass::Enemy and handle the rest directly once they are created */
            _ => Self {
                enemy: Enemy,
                name: Name::new(format!("{team} {class}")),
                team,
                class,
                scene: SceneRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one.glb")),
                ),
                hp: HitPoints(1),
                transform: Transform::from_xyz(position.x, position.y, 3.),
                rigid_body: RigidBody::Static,
                collider: Collider::rectangle(ENEMY_SIZE.x, ENEMY_SIZE.y),
                collision_events_enabled: CollisionEventsEnabled,
                collision_layers: CollisionLayers::new(
                    CollisionLayer::Enemy,
                    [CollisionLayer::PlayerProjectile],
                ),
            },
        }
    }
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct EnemyCollisionEvent {
    pub attacking_points: AttackPoints,
    pub class: EnemyClass,
    pub team: EnemyTeam,
}

impl EnemyCollisionEvent {
    pub fn new(class: EnemyClass, team: EnemyTeam, attacking_points: AttackPoints) -> Self {
        Self {
            attacking_points,
            class,
            team,
        }
    }
}

fn on_enemy_collision(
    trigger: Trigger<EnemyCollisionEvent>,
    mut commands: Commands,
    mut enemy_q: Query<(&mut HitPoints, &Transform, &EnemyClass, &EnemyTeam)>,
) {
    let Ok((mut hp, transform, class, team)) = enemy_q.get_mut(trigger.target()) else {
        warn!("Could not find just collided Enemy");
        return;
    };
    hp.0 = hp.0.saturating_sub(trigger.event().attacking_points.0);
    if hp.0 == 0 {
        commands.trigger_targets(
            EnemyDestroyedEvent {
                class: class.clone(),
                destruction_source: EnemyDestructionSource::Player,
                position: transform.translation.truncate(),
                team: team.clone(),
            },
            trigger.target(),
        );
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum EnemyDestructionSource {
    ExplosionChain,
    Player,
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct EnemyDestroyedEvent {
    pub class: EnemyClass,
    pub destruction_source: EnemyDestructionSource,
    pub position: Vec2,
    pub team: EnemyTeam,
}

fn remove_enemy_when_destroyed(trigger: Trigger<EnemyDestroyedEvent>, mut commands: Commands) {
    commands.entity(trigger.target()).despawn();
}

fn spawn_chain_when_destroyed_by_player(
    trigger: Trigger<EnemyDestroyedEvent>,
    mut commands: Commands,
) {
    let EnemyDestroyedEvent {
        class,
        destruction_source,
        position: _,
        team,
    } = trigger.event();
    if destruction_source == &EnemyDestructionSource::Player {
        if let Some(next_stage) = ExplosionChain::following_stage(class) {
            commands.spawn(ExplosionChain::new(team.clone(), next_stage));
        }
    }
}
