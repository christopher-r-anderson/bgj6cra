use bevy::prelude::*;

use crate::screens::{
    loading_screen::LoadingScreenPlugin, new_game_run::NewGameRunScreenPlugin,
    title_screen::TitleScreenPlugin,
};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LoadingScreenPlugin,
            NewGameRunScreenPlugin,
            TitleScreenPlugin,
        ));
    }
}
