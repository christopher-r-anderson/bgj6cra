use bevy::prelude::*;

use crate::{app_state::AppState, menus::main_menu};

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Title), spawn_main_menu);
    }
}

fn spawn_main_menu(commands: Commands, asset_server: Res<AssetServer>) {
    main_menu::spawn_main_menu(commands, &asset_server);
}
