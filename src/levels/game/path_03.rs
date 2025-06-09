use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Off The Beaten Path 3",
        notes: "",
        start_position: vec2(0., -340.),
        enemies: vec![
            EnemyBundle::new_wall(asset_server, vec2(-200., 245.), vec2(30., 260.)),
            EnemyBundle::new_wall(asset_server, vec2(200., 245.), vec2(30., 260.)),
            EnemyBundle::new_wall(asset_server, vec2(-200., -210.), vec2(30., 310.)),
            EnemyBundle::new_wall(asset_server, vec2(200., -210.), vec2(30., 310.)),
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(-240., 90.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(-200., 90.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(-160., 90.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(-240., -30.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(-200., -30.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(-160., -30.)),
            EnemyBundle::new_land(asset_server, vec2(0., 0.), vec2(260., 720.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(160., 90.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(200., 90.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(240., 90.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(160., -30.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(200., -30.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(240., -30.)),
        ],
    }
}
