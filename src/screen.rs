use bevy::prelude::*;

use crate::screens::{
    gameplay_screen::GameplayScreenPlugin, loading_screen::LoadingScreenPlugin,
    title_screen::TitleScreenPlugin,
};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>()
            .add_plugins(LoadingScreenPlugin)
            .add_plugins(TitleScreenPlugin)
            .add_plugins(GameplayScreenPlugin);
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Loading,
    Title,
    Gameplay,
}
