use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    enemy::{Enemy, EnemyBase, EnemyBaseCollisionEvent, EnemyCollisionEvent},
    player::{PlayerProjectile, PlayerProjectileCollisionEvent},
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_started_collisions);
    }
}

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Enemy,
    EnemyBase,
    PlayerProjectile,
}

fn print_started_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
    source_q: Query<(Has<PlayerProjectile>, Has<Enemy>, Has<EnemyBase>)>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        for entity in [entity1, entity2] {
            if let Ok((is_player_projectile, is_enemy, is_enemy_base)) = source_q.get(*entity) {
                if is_player_projectile {
                    commands.trigger_targets(PlayerProjectileCollisionEvent {}, *entity);
                } else if is_enemy {
                    commands.trigger_targets(EnemyCollisionEvent {}, *entity);
                } else if is_enemy_base {
                    commands.trigger_targets(EnemyBaseCollisionEvent {}, *entity);
                }
            }
        }
    }
}
