use bevy::prelude::*;

use crate::gameplay::{
    enemy::{EnemyBundle, EnemyClass, EnemyTeam},
    level::LevelConfig,
};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Defenders",
        notes: concat!(
            "Enemy Bases can be guarded by Defenders.\n",
            "\n",
            "You can destroy the Enemy Defenders or go straight for the Enemy Base.",
            "Destroying a base will cause a chain reaction which destroys Enemy Defenders after a short delay.",
        ),
        start_position: vec2(0., 0.),
        enemies: vec![
            EnemyBundle::new(
                asset_server,
                EnemyTeam::Alien,
                EnemyClass::Base,
                vec2(0., 330.),
            ),
            EnemyBundle::new(
                asset_server,
                EnemyTeam::Alien,
                EnemyClass::Defender,
                vec2(-60., 200.),
            ),
            EnemyBundle::new(
                asset_server,
                EnemyTeam::Alien,
                EnemyClass::Defender,
                vec2(-20., 200.),
            ),
            EnemyBundle::new(
                asset_server,
                EnemyTeam::Alien,
                EnemyClass::Defender,
                vec2(40., 0.),
            ),
        ],
    }
}
