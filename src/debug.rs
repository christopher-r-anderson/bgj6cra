use avian2d::prelude::PhysicsDebugPlugin;
use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::{gameplay::level::LevelState, screen::Screen};

const DEBUG_PHYSICS: bool = true;
const DEBUG_SCREEN_STATE: bool = false;
const DEBUG_LEVEL_STATE: bool = false;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShowAxis(false))
            .add_systems(Update, draw_axis);
        if DEBUG_PHYSICS {
            app.add_plugins(PhysicsDebugPlugin::default());
        }
        if DEBUG_SCREEN_STATE {
            app.add_systems(Update, log_transitions::<Screen>);
        }
        if DEBUG_LEVEL_STATE {
            app.add_systems(Update, log_transitions::<LevelState>);
        }
    }
}

#[derive(Resource)]
struct ShowAxis(bool);

fn draw_axis(mut gizmos: Gizmos, show_axis: Res<ShowAxis>) {
    if show_axis.0 {
        gizmos.axes_2d(Transform::default(), 100.);
    }
}
