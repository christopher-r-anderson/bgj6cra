use bevy::prelude::*;

use crate::{
    gameplay::{player::player_bundle, stage::spawn_stage},
    levels::training_01,
    screen::Screen,
};

pub struct GameplayScreenPlugin;

impl Plugin for GameplayScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    }
}

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_config = training_01::get_config(&asset_server);
    commands.spawn_batch(level_config.enemies);
    commands.spawn((
        StateScoped(Screen::Gameplay),
        player_bundle(&asset_server, level_config.start_position),
    ));
    spawn_stage(commands, &asset_server);
}
