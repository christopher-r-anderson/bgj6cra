use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Defenders",
        notes: concat!(
            "Enemy Bases can be guarded by Defenders.\n",
            "\n",
            "You can destroy the Enemy Defenders or go straight for the Enemy Base.\n",
            "Destroying a base will cause a chain reaction which destroys Enemy Defenders after a short delay.",
        ),
        start_position: vec2(0., 0.),
        enemies: vec![
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_defender(asset_server, vec2(-60., 200.)),
            EnemyBundle::new_defender(asset_server, vec2(-20., 200.)),
            EnemyBundle::new_defender(asset_server, vec2(40., 0.)),
        ],
    }
}
