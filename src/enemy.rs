use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    collisions::CollisionLayer,
    energy::{AttackPoints, HitPoints},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<EnemyClass>()
            .register_type::<EnemyTeam>()
            .add_observer(on_enemy_collision)
            .add_observer(on_enemy_destroyed)
            .add_systems(Startup, setup);
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Clone, Debug, Reflect)]
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

#[derive(Component, Clone, Debug, Reflect)]
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
                collider: Collider::rectangle(80., 30.),
                collision_events_enabled: CollisionEventsEnabled,
                collision_layers: CollisionLayers::new(
                    CollisionLayer::EnemyBase,
                    [CollisionLayer::PlayerProjectile],
                ),
            },
            EnemyClass::Enemy | _ => Self {
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
                collider: Collider::rectangle(28., 28.),
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
                position: transform.translation.truncate(),
                team: team.clone(),
            },
            trigger.target(),
        );
    }
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct EnemyDestroyedEvent {
    pub class: EnemyClass,
    pub position: Vec2,
    pub team: EnemyTeam,
}

fn on_enemy_destroyed(trigger: Trigger<EnemyDestroyedEvent>, mut commands: Commands) {
    commands.entity(trigger.target()).despawn();
}
