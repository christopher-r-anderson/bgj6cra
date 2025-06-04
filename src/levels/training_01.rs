use bevy::prelude::*;

use crate::{
    enemy::{EnemyBundle, EnemyClass, EnemyTeam},
    level::LevelConfig,
};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        start_position: vec2(0., 0.),
        enemies: vec![
            EnemyBundle::new(
                asset_server,
                EnemyTeam::Alien,
                EnemyClass::EnemyBase,
                vec2(0., 330.),
            ),
            EnemyBundle::new(
                asset_server,
                EnemyTeam::Alien,
                EnemyClass::Enemy,
                vec2(-60., 200.),
            ),
            EnemyBundle::new(
                asset_server,
                EnemyTeam::Alien,
                EnemyClass::Enemy,
                vec2(-20., 200.),
            ),
            EnemyBundle::new(
                asset_server,
                EnemyTeam::Alien,
                EnemyClass::Enemy,
                vec2(40., 0.),
            ),
        ],
    }
}
