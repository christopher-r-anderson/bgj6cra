use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    collisions::CollisionLayer,
    enemy::{EnemyClass, EnemyDestroyedEvent},
};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Explosion>()
            .add_observer(on_enemy_destroyed)
            .add_systems(Startup, setup);
    }
}

pub fn setup() {}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Explosion;

fn on_enemy_destroyed(
    trigger: Trigger<EnemyDestroyedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let EnemyDestroyedEvent {
        position,
        class,
        team,
    } = trigger.event();

    let (collider, scene) = match class {
        EnemyClass::EnemyBase => (
            Collider::rectangle(80., 30.),
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("explosions/enemy-base-explosion.glb")),
        ),
        EnemyClass::Enemy | _ => (
            Collider::rectangle(28., 28.),
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("explosions/enemy-explosion.glb")),
        ),
    };

    commands.spawn((
        Explosion,
        Name::new("EnemyExplosion"),
        SceneRoot(scene),
        Transform::from_translation(position.extend(5.)),
        RigidBody::Static,
        collider,
        CollisionEventsEnabled,
        CollisionLayers::new(CollisionLayer::EnemyExplosion, [CollisionLayer::Player]),
    ));
}
