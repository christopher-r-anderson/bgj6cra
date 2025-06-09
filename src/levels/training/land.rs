use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Land",
        notes: concat!(
            "Enemy Bases control Land.\n",
            "\n",
            "You can't destroy it and it can't destroy you.\n",
            "However, it will be destroyed in the chain reaction (after defenders) and you must survive its explosions to pass the level.",
        ),
        start_position: vec2(0., 0.),
        enemies: vec![
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_land(asset_server, vec2(0., 0.), vec2(100., 100.)),
            EnemyBundle::new_tertiary_defender(asset_server, vec2(-100., 200.)),
            EnemyBundle::new_tertiary_defender(asset_server, vec2(100., 200.)),
        ],
    }
}
