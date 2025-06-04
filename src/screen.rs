use bevy::prelude::*;

use crate::screens::{loading_screen::LoadingScreenPlugin, title_screen::TitleScreenPlugin};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>()
            .add_plugins(LoadingScreenPlugin)
            .add_plugins(TitleScreenPlugin)
            .add_systems(OnEnter(Screen::ResetGameplay), go_to_gameplay);
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Loading,
    Title,
    // Always go to ResetGameplay which will auto transition to Gameplay while allowing LevelState::Loading to run again
    ResetGameplay,
    Gameplay,
}

fn go_to_gameplay(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Gameplay);
}
