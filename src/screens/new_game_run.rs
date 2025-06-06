use bevy::prelude::*;

use crate::{
    app_state::AppState,
    gameplay::game_run::{GameRun, GameRunMode, SelectedGameRunMode},
    menus::level_select_menu::spawn_level_select_menu,
};

pub struct NewGameRunScreenPlugin;

impl Plugin for NewGameRunScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRun), spawn_game_run);
    }
}

fn spawn_game_run(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut selected_game_run_mode: ResMut<SelectedGameRunMode>,
    game_run_q: Query<Entity, With<GameRun>>,
) {
    for game_run in game_run_q {
        commands.entity(game_run).despawn();
    }
    match selected_game_run_mode.0 {
        Some(GameRunMode::Training) => {
            selected_game_run_mode.0 = None;
            commands.spawn(GameRun::new_training());
            next_state.set(AppState::ResetGameplay);
        }
        Some(GameRunMode::Game) => {
            selected_game_run_mode.0 = None;
            commands.spawn(GameRun::new_game());
            next_state.set(AppState::ResetGameplay);
        }
        Some(GameRunMode::SingleLevel) => {
            spawn_level_select_menu(commands, &asset_server);
        }
        None => {
            // TODO: do something so the user can continue
            warn!("No selected game run mode found!");
        }
    };
}
