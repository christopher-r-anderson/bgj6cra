use bevy::{prelude::*, window::CursorGrabMode};
use bevy_enhanced_input::prelude::*;

pub struct AppWindowPlugin;

impl Plugin for AppWindowPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EnhancedInputPlugin>() {
            app.add_plugins(EnhancedInputPlugin);
        }
        app.add_input_context::<Always>()
            .add_observer(binding)
            .add_observer(capture_cursor)
            .add_observer(release_cursor)
            .add_systems(Startup, setup);
    }
}

#[derive(InputContext)]
struct Always;

fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.visible = false;

    commands.spawn(Actions::<Always>::default());
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct CaptureCursor;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ReleaseCursor;

fn binding(trigger: Trigger<Binding<Always>>, mut players: Query<&mut Actions<Always>>) {
    let mut actions = players.get_mut(trigger.target()).unwrap();

    actions.bind::<CaptureCursor>().to(MouseButton::Left);
    actions.bind::<ReleaseCursor>().to(KeyCode::Escape);
}

fn capture_cursor(_trigger: Trigger<Completed<CaptureCursor>>, mut window: Single<&mut Window>) {
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.visible = false;
}

fn release_cursor(_trigger: Trigger<Completed<ReleaseCursor>>, mut window: Single<&mut Window>) {
    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;
}
