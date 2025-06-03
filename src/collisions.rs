use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    enemy::{Enemy, EnemyBase, EnemyBaseCollisionEvent, EnemyCollisionEvent},
    energy::AttackPoints,
    player::{PlayerProjectile, PlayerProjectileCollisionEvent},
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collisions);
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

fn handle_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
    projectile_q: Query<&AttackPoints, With<PlayerProjectile>>,
    hit_target_type_q: Query<(Has<Enemy>, Has<EnemyBase>)>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let (ap, projectile, hit_target, (is_enemy, is_enemy_base)) =
            match (projectile_q.get(*entity1), projectile_q.get(*entity2)) {
                (Ok(ap), Err(_)) => {
                    let Ok(target_type) = hit_target_type_q.get(*entity2) else {
                        warn!("Couldn't find type of hit target - skipping");
                        continue;
                    };
                    (ap, *entity1, *entity2, target_type)
                }
                (Err(_), Ok(ap)) => {
                    let Ok(target_type) = hit_target_type_q.get(*entity1) else {
                        warn!("Couldn't find type of hit target - skipping");
                        continue;
                    };
                    (ap, *entity2, *entity1, target_type)
                }
                (Ok(_), Ok(_)) => {
                    warn!("Both colliding entities are PlayerProjectiles - skipping");
                    continue;
                }
                (Err(_), Err(_)) => {
                    warn!("Neither colliding entities are PlayerProjectiles - skipping");
                    continue;
                }
            };
        commands.trigger_targets(PlayerProjectileCollisionEvent::default(), projectile);
        if is_enemy {
            commands.trigger_targets(EnemyCollisionEvent::new(ap.clone()), hit_target);
        } else if is_enemy_base {
            commands.trigger_targets(EnemyBaseCollisionEvent::new(ap.clone()), hit_target);
        }
    }
}
