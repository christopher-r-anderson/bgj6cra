use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Off The Beaten Path",
        notes: "",
        start_position: vec2(0., -340.),
        enemies: vec![
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_defender(asset_server, vec2(-240., 120.)),
            EnemyBundle::new_defender(asset_server, vec2(-200., 120.)),
            EnemyBundle::new_defender(asset_server, vec2(-160., 120.)),
            EnemyBundle::new_defender(asset_server, vec2(-240., -60.)),
            EnemyBundle::new_defender(asset_server, vec2(-200., -60.)),
            EnemyBundle::new_defender(asset_server, vec2(-160., -60.)),
            EnemyBundle::new_land(asset_server, vec2(0., -60.), vec2(260., 660.)),
            EnemyBundle::new_defender(asset_server, vec2(160., 120.)),
            EnemyBundle::new_defender(asset_server, vec2(200., 120.)),
            EnemyBundle::new_defender(asset_server, vec2(240., 120.)),
            EnemyBundle::new_defender(asset_server, vec2(160., -60.)),
            EnemyBundle::new_defender(asset_server, vec2(200., -60.)),
            EnemyBundle::new_defender(asset_server, vec2(240., -60.)),
        ],
    }
}
