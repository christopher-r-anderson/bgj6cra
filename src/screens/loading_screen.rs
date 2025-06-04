use bevy::prelude::*;

use crate::screen::Screen;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Loading), check_load_status);
    }
}

fn check_load_status(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Title);
}
