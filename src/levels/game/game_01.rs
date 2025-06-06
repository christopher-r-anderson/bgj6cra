use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "The Beginning",
        notes: "Here we go...",
        start_position: vec2(0., 0.),
        enemies: vec![EnemyBundle::new_base(asset_server, vec2(0., 330.))],
    }
}
