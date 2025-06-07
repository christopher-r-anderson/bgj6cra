use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "In Yer Face 2",
        notes: "",
        start_position: vec2(0., 280.),
        enemies: vec![
            EnemyBundle::new_primary_defender(asset_server, vec2(-160., 280.)),
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_wall(asset_server, vec2(0., 230.), vec2(520., 40.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(160., 280.)),
        ],
    }
}
