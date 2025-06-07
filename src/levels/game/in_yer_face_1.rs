use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "In Yer Face",
        notes: "",
        start_position: vec2(0., 280.),
        enemies: vec![
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_wall(asset_server, vec2(0., 230.), vec2(200., 40.)),
        ],
    }
}
