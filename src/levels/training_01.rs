use bevy::prelude::*;

use crate::gameplay::{
    enemy::{EnemyBundle, EnemyClass, EnemyTeam},
    level::LevelConfig,
};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        start_position: vec2(0., 0.),
        enemies: vec![EnemyBundle::new(
            asset_server,
            EnemyTeam::Alien,
            EnemyClass::EnemyBase,
            vec2(0., 330.),
        )],
    }
}
