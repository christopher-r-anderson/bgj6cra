use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Enemy Bases",
        notes: "Fire at the Enemy Base to destroy them.\n\nYou aren't safe until you have avoided their explosion.",
        start_position: vec2(0., 0.),
        enemies: vec![EnemyBundle::new_base(asset_server, vec2(0., 330.))],
    }
}
