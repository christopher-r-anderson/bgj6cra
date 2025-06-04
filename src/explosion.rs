use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    collisions::CollisionLayer,
    enemy::{
        ENEMY_BASE_SIZE, ENEMY_SIZE, Enemy, EnemyClass, EnemyDestroyedEvent,
        EnemyDestructionSource, EnemyTeam,
    },
    energy::AttackPoints,
};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Explosion>()
            .add_observer(on_enemy_destroyed)
            .add_observer(on_explosion_chain_event)
            .add_observer(on_explosion_collision)
            .add_systems(FixedUpdate, tick_explosion_chain)
            .add_systems(FixedUpdate, update_explosion);
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Explosion;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
struct ExplosionLifecycle(Timer);

fn on_enemy_destroyed(
    trigger: Trigger<EnemyDestroyedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let EnemyDestroyedEvent {
        position,
        destruction_source: _,
        class,
        team: _,
    } = trigger.event();

    let (collider, scene) = match class {
        EnemyClass::EnemyBase => (
            Collider::rectangle(ENEMY_BASE_SIZE.x, ENEMY_BASE_SIZE.y),
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("explosions/enemy-base-explosion.glb")),
        ),
        /* TODO: use this for EnemyClass::Enemy and handle the rest directly once they are created */
        _ => (
            Collider::rectangle(ENEMY_SIZE.x, ENEMY_SIZE.y),
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("explosions/enemy-explosion.glb")),
        ),
    };
    commands.spawn((
        Explosion,
        AttackPoints(1),
        class.clone(),
        ExplosionLifecycle(Timer::from_seconds(2., TimerMode::Once)),
        Name::new("EnemyExplosion"),
        SceneRoot(scene),
        Transform::from_translation(position.extend(5.)),
        RigidBody::Dynamic,
        collider,
        CollisionEventsEnabled,
        CollisionLayers::new(CollisionLayer::EnemyExplosion, [CollisionLayer::Player]),
    ));
}

#[derive(Event, Clone, Debug, Default, Reflect)]
pub struct ExplosionCollisionEvent {}

fn on_explosion_collision(trigger: Trigger<ExplosionCollisionEvent>, mut commands: Commands) {
    commands.entity(trigger.target()).despawn();
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct ExplosionChainEvent {
    class: EnemyClass,
    team: EnemyTeam,
}

impl ExplosionChainEvent {
    pub fn new(team: EnemyTeam, class: EnemyClass) -> Self {
        Self { class, team }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct ExplosionChain {
    stage: Option<EnemyClass>,
    team: EnemyTeam,
    timer: Timer,
}

impl ExplosionChain {
    pub fn following_stage(stage: &EnemyClass) -> Option<EnemyClass> {
        match stage {
            EnemyClass::EnemyBase => Some(EnemyClass::Enemy),
            EnemyClass::Enemy => Some(EnemyClass::Land),
            EnemyClass::Land => Some(EnemyClass::Projectile),
            EnemyClass::Projectile => None,
        }
    }
    pub fn new(team: EnemyTeam, stage: EnemyClass) -> Self {
        Self {
            stage: Some(stage),
            team,
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
    pub fn is_complete(&self) -> bool {
        self.stage
            .as_ref()
            .is_none_or(|stage| Self::following_stage(stage).is_none())
    }
    pub fn tick(&mut self, delta: Duration) -> Option<ExplosionChainEvent> {
        if let Some(stage) = &self.stage {
            self.timer.tick(delta);
            if self.timer.just_finished() {
                let event = ExplosionChainEvent::new(self.team.clone(), stage.clone());
                self.stage = self.stage.as_ref().and_then(Self::following_stage);
                Some(event)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn tick_explosion_chain(
    mut commands: Commands,
    time: Res<Time>,
    mut chain_q: Query<(Entity, &mut ExplosionChain)>,
) {
    for (entity, mut chain) in &mut chain_q {
        if let Some(event) = chain.tick(time.delta()) {
            commands.trigger(event);
        }
        if chain.is_complete() {
            commands.entity(entity).despawn();
        }
    }
}

fn on_explosion_chain_event(
    trigger: Trigger<ExplosionChainEvent>,
    mut commands: Commands,
    enemy_q: Query<(Entity, &EnemyTeam, &EnemyClass, &Transform), With<Enemy>>,
) {
    for (entity, team, class, transform) in &enemy_q {
        if class == &trigger.event().class && team == &trigger.event().team {
            commands.trigger_targets(
                EnemyDestroyedEvent {
                    class: class.clone(),
                    destruction_source: EnemyDestructionSource::ExplosionChain,
                    position: transform.translation.truncate(),
                    team: team.clone(),
                },
                entity,
            );
        }
    }
}

fn update_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut explosions_q: Query<
        (
            Entity,
            &mut ExplosionLifecycle,
            &mut Transform,
            &mut Collider,
            &EnemyClass,
        ),
        With<Explosion>,
    >,
) {
    for (entity, mut explosion_lifecycle, mut transform, mut collider, class) in &mut explosions_q {
        explosion_lifecycle.0.tick(time.delta());
        if explosion_lifecycle.0.just_finished() {
            commands.entity(entity).despawn();
        } else {
            transform.scale = Vec3::splat(1. + 2. * explosion_lifecycle.0.fraction());
            *collider = match class {
                EnemyClass::EnemyBase => Collider::rectangle(ENEMY_BASE_SIZE.x, ENEMY_BASE_SIZE.y),
                /* TODO: use this for EnemyClass::Enemy and handle the rest directly once they are created */
                _ => Collider::rectangle(ENEMY_SIZE.x, ENEMY_SIZE.y),
            };
        }
    }
}
