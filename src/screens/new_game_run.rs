use bevy::prelude::*;

use crate::{app_state::AppState, gameplay::game_run::GameRun};

pub struct NewGameRunScreenPlugin;

impl Plugin for NewGameRunScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRun), spawn_game_run);
    }
}

fn spawn_game_run(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    game_run_q: Query<Entity, With<GameRun>>,
) {
    for game_run in game_run_q {
        commands.entity(game_run).despawn();
    }
    commands.spawn(GameRun::new_training());
    next_state.set(AppState::ResetGameplay);
}
