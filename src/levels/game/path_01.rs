use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Off The Beaten Path",
        notes: "",
        start_position: vec2(0., -340.),
        enemies: vec![
            EnemyBundle::new_wall(asset_server, vec2(-200., 255.), vec2(30., 220.)),
            EnemyBundle::new_wall(asset_server, vec2(200., 255.), vec2(30., 220.)),
            EnemyBundle::new_wall(asset_server, vec2(-200., -225.), vec2(30., 270.)),
            EnemyBundle::new_wall(asset_server, vec2(200., -225.), vec2(30., 270.)),
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_shadow(asset_server, vec2(-240., 120.)),
            EnemyBundle::new_shadow(asset_server, vec2(-200., 120.)),
            EnemyBundle::new_shadow(asset_server, vec2(-160., 120.)),
            EnemyBundle::new_shadow(asset_server, vec2(-240., -60.)),
            EnemyBundle::new_shadow(asset_server, vec2(-200., -60.)),
            EnemyBundle::new_shadow(asset_server, vec2(-160., -60.)),
            EnemyBundle::new_land(asset_server, vec2(0., 0.), vec2(260., 720.)),
            EnemyBundle::new_shadow(asset_server, vec2(160., 120.)),
            EnemyBundle::new_shadow(asset_server, vec2(200., 120.)),
            EnemyBundle::new_shadow(asset_server, vec2(240., 120.)),
            EnemyBundle::new_shadow(asset_server, vec2(160., -60.)),
            EnemyBundle::new_shadow(asset_server, vec2(200., -60.)),
            EnemyBundle::new_shadow(asset_server, vec2(240., -60.)),
        ],
    }
}
