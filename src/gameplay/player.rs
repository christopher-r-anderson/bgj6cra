use std::time::Duration;

use avian2d::{math::*, prelude::*};
use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_enhanced_input::prelude::*;

use crate::{
    app_state::AppState,
    gameplay::{
        collisions::CollisionLayer,
        energy::{AttackPoints, HitPoints},
        level::LevelState,
        stage::{STAGE_HEIGHT, STAGE_WIDTH},
    },
};

const PLAYER_SIZE: Vec2 = Vec2::new(37.8, 38.6);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EnhancedInputPlugin>() {
            app.add_plugins(EnhancedInputPlugin);
        }
        app.register_type::<PlayerWeapon>()
            .add_input_context::<Playing>()
            .add_observer(binding)
            .add_observer(apply_movement)
            .add_observer(start_firing)
            .add_observer(completed_firing)
            .add_observer(on_spawn_player)
            .add_observer(on_player_projectile_collision)
            .add_observer(on_player_collision)
            .add_observer(on_player_destroyed)
            .add_systems(OnEnter(LevelState::Playing), add_input_context)
            .add_systems(OnExit(LevelState::Playing), remove_input_context)
            .add_systems(
                Update,
                update_player_explosion.run_if(any_with_component::<PlayerExplosion>),
            )
            .add_systems(
                FixedUpdate,
                fire_player_projectile.run_if(in_state(LevelState::Playing)),
            );
    }
}

pub fn player_bundle(asset_server: &AssetServer, position: Vec2) -> impl Bundle {
    (
        Player,
        Name::new("Player"),
        StateScoped(AppState::Gameplay),
        Speed(200.),
        HitPoints(1),
        AutoFire::new(0.2, false /* TODO: is_firing_active? */),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("player-ship/player-ship.glb")),
        ),
        RigidBody::Dynamic,
        Collider::triangle(
            vec2(0., 27.304),
            vec2(20.711, -13.904),
            vec2(-20.711, -13.904),
        ),
        CollisionEventsEnabled,
        CollisionLayers::new(CollisionLayer::Player, [CollisionLayer::EnemyExplosion]),
        Transform::from_translation(position.extend(10.)),
    )
}

#[derive(Component, Clone, Default, Debug)]
pub struct Speed(pub f32);

#[derive(Component, Clone, Default, Debug)]
pub struct Player;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = PlayerWeaponOwnedBy)]
pub struct PlayerOwnedWeapons(Vec<Entity>);

#[derive(Component, Clone, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerWeapon;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = PlayerOwnedWeapons)]
pub struct PlayerWeaponOwnedBy(Entity);

#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct PlayerProjectile;

#[derive(Component, Clone, Default, Debug, Reflect)]
struct AutoFire {
    active: bool,
    just_started: bool,
    timer: Timer,
}

impl AutoFire {
    fn new(gap_secs: f32, active: bool) -> Self {
        Self {
            active,
            just_started: true,
            timer: Timer::from_seconds(gap_secs, TimerMode::Repeating),
        }
    }
    fn just_triggered(&mut self) -> bool {
        let just_triggered = self.active && (self.just_started || self.timer.just_finished());
        self.just_started = false;
        just_triggered
    }
    fn start(&mut self) {
        self.active = true;
        self.just_started = true;
        self.timer.reset();
    }
    fn stop(&mut self) {
        self.active = false;
    }
    fn tick(&mut self, delta: Duration) {
        if self.active {
            self.timer.tick(delta);
        }
    }
}

fn on_spawn_player(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children_q: Query<&Children>,
    player_q: Query<(), With<Player>>,
    player_weapon_q: Query<(), With<PlayerWeapon>>,
) {
    let Ok(()) = player_q.get(trigger.target()) else {
        return;
    };
    for descendant in children_q.iter_descendants(trigger.target()) {
        if player_weapon_q.get(descendant).is_ok() {
            commands
                .entity(descendant)
                .insert(PlayerWeaponOwnedBy(trigger.target()));
        }
    }
}

fn fire_player_projectile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut player_q: Query<(&mut AutoFire, &Transform, &PlayerOwnedWeapons), With<Player>>,
    weapons_q: Query<&Transform, (With<PlayerWeapon>, Without<Player>)>,
) {
    for (mut auto_fire, player_transform, owned_weapons) in &mut player_q {
        auto_fire.tick(time.delta());
        if auto_fire.just_triggered() {
            for weapon in &owned_weapons.0 {
                let Ok(weapon_transform) = weapons_q.get(*weapon) else {
                    warn!("Could not find PlayerWeapon");
                    continue;
                };

                commands.spawn((
                    PlayerProjectile,
                    Name::new("Player Projectile"),
                    StateScoped(AppState::Gameplay),
                    AttackPoints(1),
                    SceneRoot(asset_server.load(
                        GltfAssetLabel::Scene(0).from_asset("projectiles/player-projectile.glb"),
                    )),
                    Transform::from_translation(
                        player_transform.translation.xy().extend(0.)
                            + weapon_transform.translation.xy().extend(0.),
                    ),
                    RigidBody::Dynamic,
                    Collider::circle(4. as Scalar),
                    CollisionEventsEnabled,
                    CollisionLayers::new(
                        CollisionLayer::PlayerProjectile,
                        [CollisionLayer::EnemyDefender, CollisionLayer::EnemyBase],
                    ),
                    LinearVelocity(vec2(0., 200.)),
                ));
            }
        }
    }
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct PlayerCollisionEvent {
    attacking_points: AttackPoints,
}

impl PlayerCollisionEvent {
    pub fn new(attacking_points: AttackPoints) -> Self {
        Self { attacking_points }
    }
}

fn on_player_collision(
    trigger: Trigger<PlayerCollisionEvent>,
    mut commands: Commands,
    mut player_q: Query<&mut HitPoints>,
) {
    let Ok(mut hp) = player_q.get_mut(trigger.target()) else {
        warn!("Could not find colliding Player's Hp");
        return;
    };
    hp.0 = hp.0.saturating_sub(trigger.event().attacking_points.0);
    if hp.0 == 0 {
        commands.trigger_targets(PlayerDestroyedEvent::default(), trigger.target());
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
struct PlayerExplosionTimer(Timer);

impl Default for PlayerExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2., TimerMode::Once))
    }
}

#[derive(Component, Debug)]
struct PlayerExplosion;

#[derive(Event, Clone, Debug, Default, Reflect)]
pub struct PlayerDestroyedEvent {}

fn on_player_destroyed(
    trigger: Trigger<PlayerDestroyedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_q: Query<&Transform, With<Player>>,
) {
    if let Ok(transform) = player_q.get(trigger.target()) {
        commands.entity(trigger.target()).despawn();
        commands.spawn((
            PlayerExplosion,
            Name::new("Player Explosion"),
            StateScoped(AppState::Gameplay),
            PlayerExplosionTimer::default(),
            SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("player-explosion/player-explosion.glb"),
            )),
            Transform::from_translation(transform.translation.truncate().extend(10.)),
        ));
    } else {
        warn!("Could not find just destroyed Player");
        // TODO: advance anyways
    }
}

fn update_player_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut player_explosion_q: Query<
        (Entity, &mut Transform, &mut PlayerExplosionTimer),
        With<PlayerExplosion>,
    >,
) {
    for (entity, mut transform, mut explosion_timer) in &mut player_explosion_q {
        explosion_timer.tick(time.delta());
        if explosion_timer.just_finished() {
            commands.entity(entity).despawn();
        } else {
            let scale = 1. + 30. * explosion_timer.fraction();
            transform.scale.x = scale;
            transform.scale.y = scale;
        }
    }
}

#[derive(Event, Clone, Debug, Default, Reflect)]
pub struct PlayerProjectileCollisionEvent {}

fn on_player_projectile_collision(
    trigger: Trigger<PlayerProjectileCollisionEvent>,
    mut commands: Commands,
) {
    commands.entity(trigger.target()).despawn();
}

fn binding(trigger: Trigger<Binding<Playing>>, mut players: Query<&mut Actions<Playing>>) {
    let mut actions = players.get_mut(trigger.target()).unwrap();
    actions
        .bind::<Move>()
        .to((
            Cardinal::wasd_keys(),
            Cardinal::arrow_keys(),
            Axial::left_stick(),
            Cardinal::dpad_buttons(),
        ))
        .with_modifiers((DeadZone::default(), DeltaScale));

    actions
        .bind::<Fire>()
        .to((KeyCode::Space, GamepadButton::South));
}

fn add_input_context(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    for player in player_q {
        commands
            .entity(player)
            .insert(Actions::<Playing>::default());
    }
}

fn remove_input_context(mut commands: Commands, player_q: Query<Entity, With<Player>>) {
    for player in player_q {
        commands.entity(player).remove::<Actions<Playing>>();
    }
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut transform, speed) = player_q.get_mut(trigger.target()).unwrap();
    let velocity = speed.0 * trigger.value;
    let stage_half_width = STAGE_WIDTH / 2. - PLAYER_SIZE.x / 2.;
    let stage_half_height = STAGE_HEIGHT / 2. - PLAYER_SIZE.y / 2.;
    transform.translation.x =
        (transform.translation.x + velocity.x).clamp(-stage_half_width, stage_half_width);
    transform.translation.y =
        (transform.translation.y + velocity.y).clamp(-stage_half_height, stage_half_height);
}

fn start_firing(trigger: Trigger<Started<Fire>>, mut player_q: Query<&mut AutoFire, With<Player>>) {
    let Ok(mut auto_fire) = player_q.get_mut(trigger.target()) else {
        warn!("Could not find Player that started firing");
        return;
    };
    auto_fire.start();
}

fn completed_firing(
    trigger: Trigger<Completed<Fire>>,
    mut player_q: Query<&mut AutoFire, With<Player>>,
) {
    let Ok(mut auto_fire) = player_q.get_mut(trigger.target()) else {
        warn!("Could not find Player that stopped firing");
        return;
    };
    auto_fire.stop();
}

#[derive(InputContext)]
struct Playing;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Fire;
