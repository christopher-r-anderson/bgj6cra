use avian2d::prelude::*;
use bevy::prelude::*;

use crate::collisions::CollisionLayer;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<EnemyBase>()
            .register_type::<EnemyOne>()
            .register_type::<EnemyOneBase>()
            .add_observer(on_enemy_collision)
            .add_observer(on_enemy_base_collision)
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
        Transform::from_xyz(-20., 200., 3.),
        RigidBody::Static,
        Collider::rectangle(28., 28.),
        CollisionEventsEnabled,
        CollisionLayers::new(CollisionLayer::Enemy, [CollisionLayer::PlayerProjectile]),
    ));
}

#[derive(Event, Clone, Debug, Default, Reflect)]
pub struct EnemyCollisionEvent {}

fn on_enemy_collision(trigger: Trigger<EnemyCollisionEvent>) {
    info!("Enemy Hit: {}", trigger.target());
}

#[derive(Event, Clone, Debug, Default, Reflect)]
pub struct EnemyBaseCollisionEvent {}

fn on_enemy_base_collision(trigger: Trigger<EnemyBaseCollisionEvent>) {
    info!("Enemy Base Hit: {}", trigger.target());
}
