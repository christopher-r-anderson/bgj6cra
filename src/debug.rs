use avian2d::prelude::PhysicsDebugPlugin;
use bevy::prelude::*;

const DEBUG_PHYSICS: bool = true;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShowAxis(false))
            .add_systems(Update, draw_axis);
        if DEBUG_PHYSICS {
            app.add_plugins(PhysicsDebugPlugin::default());
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
