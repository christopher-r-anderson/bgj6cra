use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    app_state::AppState,
    gameplay::{
        collisions::CollisionLayer,
        enemy::{
            ENEMY_BASE_SIZE, ENEMY_DEFENDER_SIZE, ENEMY_LAND_SIZE, ENEMY_SHADOW_SIZE, Enemy,
            EnemyClass, EnemyClassWave, EnemyDestroyedEvent, EnemyDestructionSource, EnemyTeam,
        },
        energy::AttackPoints,
        level::LevelState,
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
        wave: _,
    } = trigger.event();

    let (collider, scene) = match class {
        EnemyClass::Base => (
            Collider::rectangle(ENEMY_BASE_SIZE.x, ENEMY_BASE_SIZE.y),
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("explosions/enemy-base-explosion.glb")),
        ),
        EnemyClass::Defender | EnemyClass::Shadow => (
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
        class.clone(),
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
    class: EnemyClass,
    team: EnemyTeam,
    wave: EnemyClassWave,
}

impl ExplosionChainEvent {
    pub fn new(team: EnemyTeam, class: EnemyClass, wave: EnemyClassWave) -> Self {
        Self { class, team, wave }
    }
}

// HACK: use a better method, this is just a quick, temporary way to know if we should chain multiple Waves
#[derive(Debug, Default)]
pub struct DefenderWavesInLevel {
    primary: bool,
    secondary: bool,
    tertiary: bool,
}

impl From<Vec<&EnemyClassWave>> for DefenderWavesInLevel {
    fn from(waves: Vec<&EnemyClassWave>) -> Self {
        let mut statuses = Self::default();
        for wave in waves {
            match wave {
                EnemyClassWave::Primary => statuses.primary = true,
                EnemyClassWave::Secondary => statuses.secondary = true,
                EnemyClassWave::Tertiary => statuses.tertiary = true,
            };
        }
        statuses
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct ExplosionChain {
    stage: Option<(EnemyClass, EnemyClassWave)>,
    team: EnemyTeam,
    timer: Timer,
}

impl ExplosionChain {
    pub fn following_stage(
        class: &EnemyClass,
        wave: &EnemyClassWave,
        defender_waves: &DefenderWavesInLevel,
    ) -> Option<(EnemyClass, EnemyClassWave)> {
        match class {
            EnemyClass::Base => Some((EnemyClass::Shadow, EnemyClassWave::Primary)),
            EnemyClass::Shadow => Some((EnemyClass::Defender, EnemyClassWave::Primary)),
            EnemyClass::Defender => {
                // TODO: This needs a better approach. Should probably be different classes, but that will require not waiting
                //       for any non existent classes. See also DefenderWavesInLevel
                match (
                    wave,
                    defender_waves.primary,
                    defender_waves.secondary,
                    defender_waves.tertiary,
                ) {
                    (EnemyClassWave::Primary, _, true, _) => {
                        Some((EnemyClass::Defender, EnemyClassWave::Secondary))
                    }
                    (EnemyClassWave::Secondary, _, _, true) => {
                        Some((EnemyClass::Defender, EnemyClassWave::Tertiary))
                    }
                    _ => Some((EnemyClass::Land, EnemyClassWave::Primary)),
                }
            }
            // EnemyClass::Land => Some(EnemyClass::Projectile),
            // EnemyClass::Projectile => None,
            EnemyClass::Land => None,
            // TODO: better types so we don't pepper around unreachable! for this
            EnemyClass::Wall => unreachable!("Enemy Walls can't be destroyed"),
        }
    }
    pub fn new(team: EnemyTeam, stage: EnemyClass, wave: EnemyClassWave) -> Self {
        Self {
            stage: Some((stage, wave)),
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
        defender_waves: &DefenderWavesInLevel,
    ) -> Option<ExplosionChainEvent> {
        if let Some(stage) = &self.stage {
            self.timer.tick(delta);
            if self.timer.just_finished() {
                let event =
                    ExplosionChainEvent::new(self.team.clone(), stage.0.clone(), stage.1.clone());
                self.stage = self
                    .stage
                    .as_ref()
                    .and_then(|(class, wave)| Self::following_stage(class, wave, defender_waves));
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

    waves_q: Query<(&EnemyClass, &EnemyClassWave)>,
) {
    let defender_waves = waves_q
        .iter()
        .filter_map(|(class, wave)| {
            if class == &EnemyClass::Defender {
                Some(wave)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into();
    for (entity, mut chain) in &mut chain_q {
        if let Some(event) = chain.tick(time.delta(), &defender_waves) {
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
    enemy_q: Query<(Entity, &EnemyTeam, &EnemyClass, &EnemyClassWave, &Transform), With<Enemy>>,
) {
    for (entity, team, class, wave, transform) in &enemy_q {
        if class == &trigger.event().class
            && team == &trigger.event().team
            && wave == &trigger.event().wave
        {
            commands.trigger_targets(
                EnemyDestroyedEvent {
                    class: class.clone(),
                    destruction_source: EnemyDestructionSource::ExplosionChain,
                    position: transform.translation.truncate(),
                    scale: transform.scale.truncate(),
                    team: team.clone(),
                    wave: wave.clone(),
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
                EnemyClass::Defender => ENEMY_DEFENDER_SIZE,
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
