use bevy::prelude::*;

use crate::screens::{loading_screen::LoadingScreenPlugin, title_screen::TitleScreenPlugin};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingScreenPlugin)
            .add_plugins(TitleScreenPlugin);
    }
}
