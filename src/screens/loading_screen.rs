use bevy::prelude::*;

use crate::app_state::AppState;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), check_load_status);
    }
}

fn check_load_status(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Title);
}
