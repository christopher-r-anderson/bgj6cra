use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    app_state::AppState,
    gameplay::{
        collisions::CollisionLayer,
        enemy::{
            ENEMY_BASE_SIZE, ENEMY_DEFENDER_SIZE, ENEMY_LAND_SIZE, ENEMY_SHADOW_SIZE, Enemy,
            EnemyClass, EnemyDestroyedEvent, EnemyDestructionSource, EnemyTeam,
        },
        energy::AttackPoints,
        level::{LevelState, LevelStats},
    },
};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Explosion>()
            .add_observer(on_enemy_destroyed)
            .add_observer(on_explosion_chain_event)
            .add_systems(
                FixedUpdate,
                (tick_explosion_chain, update_explosion).run_if(in_state(LevelState::Playing)),
            );
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Explosion;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
struct ExplosionLifecycle(Timer);

#[derive(Component, Debug, Clone)]
struct SourceScale(Vec2);

fn on_enemy_destroyed(
    trigger: Trigger<EnemyDestroyedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let EnemyDestroyedEvent {
        class,
        destruction_source: _,
        position,
        scale,
        team: _,
    } = trigger.event();

    let (collider, scene) = match class {
        EnemyClass::Base => (
            Collider::rectangle(ENEMY_BASE_SIZE.x, ENEMY_BASE_SIZE.y),
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("explosions/enemy-base-explosion.glb")),
        ),
        EnemyClass::DefenderOne
        | EnemyClass::DefenderTwo
        | EnemyClass::DefenderThree
        | EnemyClass::Shadow => (
            Collider::rectangle(ENEMY_DEFENDER_SIZE.x, ENEMY_DEFENDER_SIZE.y),
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("explosions/enemy-explosion.glb")),
        ),
        EnemyClass::Land => (
            Collider::rectangle(ENEMY_LAND_SIZE.x, ENEMY_LAND_SIZE.y),
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("explosions/enemy-land-explosion.glb")),
        ),
        EnemyClass::Wall => unreachable!("Walls can't be destroyed"),
    };
    commands.spawn((
        Explosion,
        StateScoped(AppState::Gameplay),
        AttackPoints(1),
        *class,
        ExplosionLifecycle(Timer::from_seconds(1., TimerMode::Once)),
        Name::new("EnemyExplosion"),
        SceneRoot(scene),
        SourceScale(*scale),
        Transform::from_translation(position.extend(5.)),
        RigidBody::Dynamic,
        collider,
        CollisionEventsEnabled,
        CollisionLayers::new(CollisionLayer::EnemyExplosion, [CollisionLayer::Player]),
    ));
}

#[derive(Event, Clone, Debug, Default, Reflect)]
pub struct ExplosionCollisionEvent {}

#[derive(Event, Clone, Debug, Reflect)]
pub struct ExplosionChainEvent {
    pub class: EnemyClass,
    pub team: EnemyTeam,
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
    pub fn following_class(class: &EnemyClass, level_stats: &LevelStats) -> Option<EnemyClass> {
        let mut classes = EnemyClass::in_order()
            .into_iter()
            .skip_while(|current| current != class)
            .skip(1);
        classes.find(|&class| level_stats.original_enemy_counts.started_with_enemy(&class))
    }
    pub fn new(team: EnemyTeam, stage: EnemyClass) -> Self {
        Self {
            stage: Some(stage),
            team,
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
    pub fn is_complete(&self) -> bool {
        self.stage.as_ref().is_none()
    }
    pub fn tick(
        &mut self,
        delta: Duration,
        level_stats: &LevelStats,
    ) -> Option<ExplosionChainEvent> {
        if let Some(class) = &self.stage {
            self.timer.tick(delta);
            if self.timer.just_finished() {
                let event = ExplosionChainEvent::new(self.team, *class);
                self.stage = self
                    .stage
                    .as_ref()
                    .and_then(|class| Self::following_class(class, level_stats));
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
    level_stats: Single<&LevelStats>,
) {
    for (entity, mut chain) in &mut chain_q {
        if let Some(event) = chain.tick(time.delta(), &level_stats) {
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
    for (entity, &team, &class, transform) in &enemy_q {
        if class == trigger.event().class && team == trigger.event().team {
            commands.trigger_targets(
                EnemyDestroyedEvent {
                    class,
                    destruction_source: EnemyDestructionSource::ExplosionChain,
                    position: transform.translation.truncate(),
                    scale: transform.scale.truncate(),
                    team,
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
            &SourceScale,
            &EnemyClass,
        ),
        With<Explosion>,
    >,
) {
    for (entity, mut explosion_lifecycle, mut transform, source_scale, class) in &mut explosions_q {
        explosion_lifecycle.0.tick(time.delta());
        if explosion_lifecycle.0.just_finished() {
            commands.entity(entity).despawn();
        } else {
            let mesh_size = match class {
                EnemyClass::Base => ENEMY_BASE_SIZE,
                EnemyClass::DefenderOne => ENEMY_DEFENDER_SIZE,
                EnemyClass::DefenderTwo => ENEMY_DEFENDER_SIZE,
                EnemyClass::DefenderThree => ENEMY_DEFENDER_SIZE,
                EnemyClass::Land => ENEMY_LAND_SIZE,
                EnemyClass::Shadow => ENEMY_SHADOW_SIZE,
                EnemyClass::Wall => unreachable!("Enemy Walls can't be destroyed"),
            };
            let pixel_in_scale = 1. / mesh_size;
            transform.scale = (source_scale.0
                + (pixel_in_scale * 80. * explosion_lifecycle.0.fraction()))
            .extend(1.);
        }
    }
}
