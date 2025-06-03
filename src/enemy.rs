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
            .register_type::<EnemyBase>()
            .register_type::<EnemyOne>()
            .register_type::<EnemyOneBase>()
            .add_observer(on_enemy_collision)
            .add_observer(on_enemy_destroyed)
            .add_observer(on_enemy_base_collision)
            .add_observer(on_enemy_base_destroyed)
            .add_systems(Startup, setup);
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct EnemyBase;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct EnemyOneBase;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct EnemyOne;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        EnemyBase,
        EnemyOneBase,
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
        EnemyOne,
        Name::new("EnemyOne"),
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
        EnemyOne,
        Name::new("EnemyOne"),
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
    attacking_points: AttackPoints,
}

impl EnemyCollisionEvent {
    pub fn new(attacking_points: AttackPoints) -> Self {
        Self { attacking_points }
    }
}

fn on_enemy_collision(
    trigger: Trigger<EnemyCollisionEvent>,
    mut commands: Commands,
    mut enemy_q: Query<&mut HitPoints>,
) {
    let Ok(mut hp) = enemy_q.get_mut(trigger.target()) else {
        warn!("Could not find just collided Enemy");
        return;
    };
    hp.0 = hp.0.saturating_sub(trigger.event().attacking_points.0);
    if hp.0 == 0 {
        commands.trigger_targets(EnemyDestroyedEvent {}, trigger.target());
    }
}

#[derive(Event, Clone, Debug, Default, Reflect)]
pub struct EnemyDestroyedEvent {}

fn on_enemy_destroyed(trigger: Trigger<EnemyDestroyedEvent>, mut commands: Commands) {
    commands.entity(trigger.target()).despawn();
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct EnemyBaseCollisionEvent {
    attacking_points: AttackPoints,
}

impl EnemyBaseCollisionEvent {
    pub fn new(attacking_points: AttackPoints) -> Self {
        Self { attacking_points }
    }
}

fn on_enemy_base_collision(
    trigger: Trigger<EnemyBaseCollisionEvent>,
    mut commands: Commands,
    mut enemy_base_q: Query<&mut HitPoints>,
) {
    let Ok(mut hp) = enemy_base_q.get_mut(trigger.target()) else {
        warn!("Could not find just collided EnemyBase");
        return;
    };
    hp.0 = hp.0.saturating_sub(trigger.event().attacking_points.0);
    if hp.0 == 0 {
        commands.trigger_targets(EnemyDestroyedEvent {}, trigger.target());
    }
}

#[derive(Event, Clone, Debug, Default, Reflect)]
pub struct EnemyBaseDestroyedEvent {}

fn on_enemy_base_destroyed(trigger: Trigger<EnemyBaseDestroyedEvent>, mut commands: Commands) {
    commands.entity(trigger.target()).despawn();
}
