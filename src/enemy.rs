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
    commands.spawn((
        EnemyTeam::Alien,
        EnemyClass::EnemyBase,
        Name::new("EnemyOneBase"),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one-base.glb")),
        ),
        HitPoints(3),
        Transform::from_xyz(0., 330., 2.),
        RigidBody::Static,
        Collider::rectangle(80., 30.),
        CollisionEventsEnabled,
        CollisionLayers::new(
            CollisionLayer::EnemyBase,
            [CollisionLayer::PlayerProjectile],
        ),
    ));
    commands.spawn((
        Enemy,
        EnemyTeam::Alien,
        EnemyClass::Enemy,
        Name::new("Alien Enemy 1"),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one.glb"))),
        HitPoints(1),
        Transform::from_xyz(-60., 200., 3.),
        RigidBody::Static,
        Collider::rectangle(28., 28.),
        CollisionEventsEnabled,
        CollisionLayers::new(CollisionLayer::Enemy, [CollisionLayer::PlayerProjectile]),
    ));
    commands.spawn((
        Enemy,
        EnemyTeam::Alien,
        EnemyClass::Enemy,
        Name::new("Alien Enemy 2"),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one.glb"))),
        HitPoints(1),
        Transform::from_xyz(-20., 200., 3.),
        RigidBody::Static,
        Collider::rectangle(28., 28.),
        CollisionEventsEnabled,
        CollisionLayers::new(CollisionLayer::Enemy, [CollisionLayer::PlayerProjectile]),
    ));
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
