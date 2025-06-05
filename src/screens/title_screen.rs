use bevy::prelude::*;

use crate::{menus::main_menu, screen::Screen};

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Title), spawn_main_menu);
    }
}

fn spawn_main_menu(commands: Commands, asset_server: Res<AssetServer>) {
    main_menu::spawn_main_menu(commands, &asset_server);
}
