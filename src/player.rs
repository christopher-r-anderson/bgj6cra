use bevy::{prelude::*, window::CursorGrabMode};
use bevy_enhanced_input::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<Playing>()
            .add_observer(binding)
            .add_observer(apply_movement)
            .add_observer(capture_cursor)
            .add_observer(release_cursor)
            .add_systems(Startup, setup);
    }
}

#[derive(Component, Clone, Default, Debug)]
pub struct Player;

#[derive(Component, Clone, Default, Debug)]
pub struct Speed(pub f32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut window: Single<&mut Window>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.visible = false;

    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Speed(600.),
        Actions::<Playing>::default(),
        Mesh2d(meshes.add(Triangle2d::new(
            vec2(0., 12.),
            vec2(-12., -12.),
            vec2(12., -12.),
        ))),
        MeshMaterial2d(materials.add(Color::srgb(100., 100., 255.))),
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
        .with_modifiers((
            DeadZone::default(),
            DeltaScale,
        ));

    actions.bind::<CaptureCursor>().to(MouseButton::Left);
    actions.bind::<ReleaseCursor>().to(KeyCode::Escape);
}

fn apply_movement(
    trigger: Trigger<Fired<Move>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut transform, speed) = player_q.get_mut(trigger.target()).unwrap();
    let velocity = speed.0 * trigger.value;
    transform.translation += velocity.extend(0.0);
}

fn capture_cursor(_trigger: Trigger<Completed<CaptureCursor>>, mut window: Single<&mut Window>) {
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.visible = false;
}

fn release_cursor(_trigger: Trigger<Completed<ReleaseCursor>>, mut window: Single<&mut Window>) {
    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;
}

#[derive(InputContext)]
struct Playing;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct CaptureCursor;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ReleaseCursor;
