use bevy::prelude::*;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(OnEnter(AppState::ResetGameplay), go_to_gameplay)
            .add_systems(OnEnter(AppState::ResetGameRun), go_to_game_run);
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum AppState {
    #[default]
    Loading,
    Title,
    ResetGameRun,
    GameRun,
    // Always go to ResetGameplay which will auto transition to Gameplay while allowing LevelState::Loading to run again
    ResetGameplay,
    Gameplay,
}

fn go_to_gameplay(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Gameplay);
}

fn go_to_game_run(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::GameRun);
}
