use avian2d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::{
    enemy::{Enemy, EnemyClass, EnemyCollisionEvent, EnemyTeam},
    energy::AttackPoints,
    explosion::{Explosion, ExplosionCollisionEvent},
    level::LevelState,
    player::{PlayerCollisionEvent, PlayerProjectile, PlayerProjectileCollisionEvent},
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_enemy_collisions,
                handle_explosion_collisions,
                handle_player_projectile_collisions,
            )
                .run_if(in_state(LevelState::Playing)),
        );
    }
}

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    EnemyBase,
    EnemyDefender,
    EnemyExplosion,
    EnemyLand,
    EnemyShadow,
    EnemyWall,
    Player,
    PlayerProjectile,
}

fn handle_player_projectile_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
    projectile_q: Query<&AttackPoints, With<PlayerProjectile>>,
    hit_target_type_q: Query<(&EnemyClass, &EnemyTeam)>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let (projectile, hit_target, enemy_collision_event) =
            match (projectile_q.get(*entity1), projectile_q.get(*entity2)) {
                (Ok(ap), Err(_)) => {
                    let Ok((target_class, target_team)) = hit_target_type_q.get(*entity2) else {
                        warn!("Couldn't find enemy class and team of hit target - skipping");
                        continue;
                    };
                    (
                        *entity1,
                        *entity2,
                        EnemyCollisionEvent::new(*target_class, *target_team, ap.clone()),
                    )
                }
                (Err(_), Ok(ap)) => {
                    let Ok((target_class, target_team)) = hit_target_type_q.get(*entity1) else {
                        warn!("Couldn't find enemy class and team of hit target - skipping");
                        continue;
                    };
                    (
                        *entity2,
                        *entity1,
                        EnemyCollisionEvent::new(*target_class, *target_team, ap.clone()),
                    )
                }
                _ => {
                    continue;
                }
            };
        commands.trigger_targets(PlayerProjectileCollisionEvent::default(), projectile);
        if enemy_collision_event.class != EnemyClass::Wall {
            commands.trigger_targets(enemy_collision_event, hit_target);
        }
    }
}

fn handle_explosion_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
    explosion_q: Query<&AttackPoints, With<Explosion>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let (explosion, player, ap) = match (explosion_q.get(*entity1), explosion_q.get(*entity2)) {
            (Ok(ap), Err(_)) => (*entity1, *entity2, ap.clone()),
            (Err(_), Ok(ap)) => (*entity2, *entity1, ap.clone()),
            _ => {
                continue;
            }
        };
        commands.trigger_targets(PlayerCollisionEvent::new(ap), player);
        commands.trigger_targets(ExplosionCollisionEvent::default(), explosion);
    }
}

fn handle_enemy_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
    enemy_q: Query<&AttackPoints, With<Enemy>>,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        info!("Collision started");
        let (enemy, player, ap) = match (enemy_q.get(*entity1), enemy_q.get(*entity2)) {
            (Ok(ap), Err(_)) => (*entity1, *entity2, ap.clone()),
            (Err(_), Ok(ap)) => (*entity2, *entity1, ap.clone()),
            _ => {
                continue;
            }
        };
        commands.trigger_targets(PlayerCollisionEvent::new(ap), player);
        commands.trigger_targets(ExplosionCollisionEvent::default(), enemy);
    }
}
