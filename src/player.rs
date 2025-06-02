use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_enhanced_input::prelude::*;

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
            .add_observer(on_spawn_player)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (update_player_projectile_positions, fire_player_projectile).chain(),
            );
    }
}

#[derive(Component, Clone, Default, Debug)]
pub struct Player;

#[derive(Component, Clone, Default, Debug)]
pub struct Speed(pub f32);

#[derive(Component, Clone, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerWeapon;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerWeaponOwner(Entity);

#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct PlayerProjectile;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Name::new("Player"),
        Speed(200.),
        Actions::<Playing>::default(),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("player-ship/player-ship.glb")),
        ),
        Transform::default(),
    ));
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
                .insert(PlayerWeaponOwner(trigger.target()));
        }
    }
}

fn fire_player_projectile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    player_weapon_q: Query<(&Transform, &PlayerWeaponOwner)>,
    player_q: Query<&Transform, Without<PlayerWeaponOwner>>,
    mut fire_timer: Local<Option<Timer>>,
) {
    let timer = fire_timer.get_or_insert(Timer::from_seconds(0.2, TimerMode::Repeating));
    timer.tick(time.delta());
    if timer.just_finished() {
        for (weapon_transform, owner) in &player_weapon_q {
            let Ok(player_transform) = player_q.get(owner.0) else {
                warn!("Could not find Weapon's owner");
                continue;
            };
            commands.spawn((
                PlayerProjectile,
                SceneRoot(asset_server.load(
                    GltfAssetLabel::Scene(0).from_asset("projectiles/player-projectile.glb"),
                )),
                Transform::from_translation(
                    player_transform.translation.xy().extend(0.)
                        + weapon_transform.translation.xy().extend(0.),
                ),
            ));
        }
    }
}

fn update_player_projectile_positions(
    time: Res<Time>,
    mut projectile_q: Query<&mut Transform, With<PlayerProjectile>>,
) {
    for mut projectile_transform in &mut projectile_q {
        projectile_transform.translation.y += 250. * time.delta_secs();
    }
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
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut transform, speed) = player_q.get_mut(trigger.target()).unwrap();
    let velocity = speed.0 * trigger.value;
    transform.translation += velocity.extend(0.0);
}

#[derive(InputContext)]
struct Playing;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;
