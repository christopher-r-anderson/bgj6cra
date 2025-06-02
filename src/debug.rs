use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShowAxis(false))
            .add_systems(Update, draw_axis);
    }
}

#[derive(Resource)]
struct ShowAxis(bool);

fn draw_axis(mut gizmos: Gizmos, show_axis: Res<ShowAxis>) {
    if show_axis.0 {
        gizmos.axes_2d(Transform::default(), 100.);
    }
}
