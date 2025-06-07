use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "In Yer Face 3",
        notes: "",
        start_position: vec2(0., 280.),
        enemies: vec![
            EnemyBundle::new_defender(asset_server, vec2(-210., 280.)),
            EnemyBundle::new_land(asset_server, vec2(-130., 310.), vec2(100., 100.)),
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_wall(asset_server, vec2(0., 230.), vec2(520., 40.)),
            EnemyBundle::new_land(asset_server, vec2(130., 310.), vec2(100., 100.)),
            EnemyBundle::new_defender(asset_server, vec2(210., 280.)),
        ],
    }
}
