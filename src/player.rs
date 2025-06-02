use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EnhancedInputPlugin>() {
            app.add_plugins(EnhancedInputPlugin);
        }
        app.add_input_context::<Playing>()
            .add_observer(binding)
            .add_observer(apply_movement)
            .add_systems(Startup, setup);
    }
}

#[derive(Component, Clone, Default, Debug)]
pub struct Player;

#[derive(Component, Clone, Default, Debug)]
pub struct Speed(pub f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Speed(200.),
        Actions::<Playing>::default(),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("player-ship/player-ship.glb")),
        ),
        Transform::default(),
    ));
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
