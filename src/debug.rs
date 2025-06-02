use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_axis);
    }
}

fn draw_axis(mut gizmos: Gizmos) {
    gizmos.axes_2d(Transform::default(), 100.)
}
