use avian2d::prelude::*;
use bevy::prelude::*;

use crate::gameplay::{
    enemy::{EnemyClass, EnemyCollisionEvent, EnemyTeam},
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
    Enemy,
    EnemyBase,
    EnemyExplosion,
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
                        EnemyCollisionEvent::new(
                            target_class.clone(),
                            target_team.clone(),
                            ap.clone(),
                        ),
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
                        EnemyCollisionEvent::new(
                            target_class.clone(),
                            target_team.clone(),
                            ap.clone(),
                        ),
                    )
                }
                _ => {
                    continue;
                }
            };
        commands.trigger_targets(PlayerProjectileCollisionEvent::default(), projectile);
        commands.trigger_targets(enemy_collision_event, hit_target);
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
