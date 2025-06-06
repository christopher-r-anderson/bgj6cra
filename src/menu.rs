use bevy::{
    input::InputSystem,
    input_focus::{InputDispatchPlugin, InputFocus, directional_navigation::*},
    math::CompassOctant,
    prelude::*,
};
use bevy_flair::prelude::*;
use std::collections::HashSet;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<DirectionalNavigationPlugin>() {
            app.add_plugins(DirectionalNavigationPlugin);
        }
        if !app.is_plugin_added::<FlairPlugin>() {
            app.add_plugins(FlairPlugin);
        }
        if !app.is_plugin_added::<InputDispatchPlugin>() {
            app.add_plugins(InputDispatchPlugin);
        }

        // Navigation
        app.init_resource::<ActionState>()
            .add_event::<ButtonActivate>()
            .add_observer(on_click_button_observer)
            .add_observer(on_hover_button_observer)
            .add_systems(
                PreUpdate,
                (process_inputs, navigate).chain().after(InputSystem),
            )
            .add_systems(Update, interact_with_focused_button);

        // Navigate children
        app.add_systems(First, add_children_to_navigable_map);
    }
}

#[derive(Debug, Component, Copy, Clone)]
pub struct NavigableChildren {
    pub looping: bool,
    pub direction: CompassOctant,
}

impl Default for NavigableChildren {
    fn default() -> Self {
        Self {
            looping: true,
            direction: CompassOctant::South,
        }
    }
}

fn add_children_to_navigable_map(
    mut navigation_map: ResMut<DirectionalNavigationMap>,
    has_button_query: Query<Has<Button>>,
    added_navigable_children_query: Query<
        (&NavigableChildren, &Children),
        Added<NavigableChildren>,
    >,
) {
    for (navigable, children) in &added_navigable_children_query {
        let filtered_children = children
            .iter()
            .filter(|c| has_button_query.get(*c).unwrap_or(false))
            .collect::<Vec<_>>();

        if navigable.looping {
            navigation_map.add_looping_edges(&filtered_children, navigable.direction);
        } else {
            navigation_map.add_edges(&filtered_children, navigable.direction);
        }
    }
}

// The indirection between inputs and actions allows us to easily remap inputs
// and handle multiple input sources (keyboard, gamepad, etc.) in our game
#[derive(Debug, PartialEq, Eq, Hash)]
enum DirectionalNavigationAction {
    Up,
    Down,
    Select,
}

impl DirectionalNavigationAction {
    fn variants() -> Vec<Self> {
        vec![
            DirectionalNavigationAction::Up,
            DirectionalNavigationAction::Down,
            DirectionalNavigationAction::Select,
        ]
    }

    fn keycodes(&self) -> Vec<KeyCode> {
        match self {
            DirectionalNavigationAction::Up => vec![KeyCode::ArrowUp],
            DirectionalNavigationAction::Down => vec![KeyCode::ArrowDown],
            DirectionalNavigationAction::Select => vec![KeyCode::Enter, KeyCode::Space],
        }
    }

    fn gamepad_buttons(&self) -> Vec<GamepadButton> {
        match self {
            DirectionalNavigationAction::Up => vec![GamepadButton::DPadUp],
            DirectionalNavigationAction::Down => vec![GamepadButton::DPadDown],
            // This is the "A" button on an Xbox controller,
            // and is conventionally used as the "Select" / "Interact" button in many games
            DirectionalNavigationAction::Select => vec![GamepadButton::South, GamepadButton::Start],
        }
    }
}

// This keeps track of the inputs that are currently being pressed
#[derive(Default, Resource)]
struct ActionState {
    pressed_actions: HashSet<DirectionalNavigationAction>,
}

fn process_inputs(
    mut action_state: ResMut<ActionState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepad_input: Query<&Gamepad>,
) {
    // Reset the set of pressed actions each frame
    // to ensure that we only process each action once
    action_state.pressed_actions.clear();

    for action in DirectionalNavigationAction::variants() {
        // Use just_pressed to ensure that we only process each action once
        // for each time it is pressed
        if keyboard_input.any_just_pressed(action.keycodes()) {
            action_state.pressed_actions.insert(action);
        }
    }

    // We're treating this like a single-player game:
    // if multiple gamepads are connected, we don't care which one is being used
    for gamepad in gamepad_input.iter() {
        for action in DirectionalNavigationAction::variants() {
            // Unlike keyboard input, gamepads are bound to a specific controller
            if gamepad.any_just_pressed(action.gamepad_buttons()) {
                action_state.pressed_actions.insert(action);
            }
        }
    }
}

fn navigate(action_state: Res<ActionState>, mut directional_navigation: DirectionalNavigation) {
    let net_north_south = action_state
        .pressed_actions
        .contains(&DirectionalNavigationAction::Up) as i8
        - action_state
            .pressed_actions
            .contains(&DirectionalNavigationAction::Down) as i8;

    // Compute the direction that the user is trying to navigate in
    let maybe_direction = match net_north_south {
        1 => Some(CompassOctant::North),
        -1 => Some(CompassOctant::South),
        _ => None,
    };

    if let Some(direction) = maybe_direction {
        if let Err(e) = directional_navigation.navigate(direction) {
            error!("Navigation failed: {e}");
        }
    }
}

#[derive(Debug, Default, Event)]
pub struct ButtonActivate;

fn interact_with_focused_button(
    mut commands: Commands,
    action_state: Res<ActionState>,
    input_focus: Res<InputFocus>,
) {
    if action_state
        .pressed_actions
        .contains(&DirectionalNavigationAction::Select)
    {
        if let Some(focused_entity) = input_focus.0 {
            commands.trigger_targets(ButtonActivate, focused_entity);
        }
    }
}

fn on_click_button_observer(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    has_button_query: Query<Has<Button>>,
) {
    let entity = trigger.target();
    if has_button_query.get(entity).unwrap() {
        commands.trigger_targets(ButtonActivate, entity);
    }
}

fn on_hover_button_observer(
    trigger: Trigger<Pointer<Over>>,
    mut focus: ResMut<InputFocus>,
    has_button_query: Query<Has<Button>>,
) {
    let entity = trigger.target();
    if has_button_query.get(entity).unwrap_or(false) {
        focus.set(entity);
    }
}

pub fn button(text: &'static str) -> impl Bundle {
    (Button, Children::spawn_one(Text::new(text)))
}
