use bevy::prelude::*;

use crate::{
    app_state::AppState,
    gameplay::game_run::{GameRun, SelectedGameRunMode},
};

pub struct NewGameRunScreenPlugin;

impl Plugin for NewGameRunScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRun), spawn_game_run);
    }
}

fn spawn_game_run(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut selected_game_run_mode: ResMut<SelectedGameRunMode>,
    game_run_q: Query<Entity, With<GameRun>>,
) {
    for game_run in game_run_q {
        commands.entity(game_run).despawn();
    }
    let new_game_run = match &*selected_game_run_mode {
        SelectedGameRunMode::None => {
            warn!("No selected game run mode found!");
            return;
            // TODO: do something so the user can continue
        }
        SelectedGameRunMode::Training => GameRun::new_training(),
        SelectedGameRunMode::Game => GameRun::new_game(),
    };
    *selected_game_run_mode = SelectedGameRunMode::None;
    commands.spawn(new_game_run);
    next_state.set(AppState::ResetGameplay);
}
