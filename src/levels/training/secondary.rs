use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Additional Waves",
        notes: "Icons inside Enemy Defenders indicate which defense Wave they are in.\n\nDefender Waves will explode in order.",
        start_position: vec2(0., 0.),
        enemies: vec![
            EnemyBundle::new_primary_defender(asset_server, vec2(-100., 200.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(100., 200.)),
            EnemyBundle::new_secondary_defender(asset_server, vec2(-100., 0.)),
            EnemyBundle::new_secondary_defender(asset_server, vec2(100., 0.)),
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_tertiary_defender(asset_server, vec2(-100., -200.)),
            EnemyBundle::new_tertiary_defender(asset_server, vec2(100., -200.)),
        ],
    }
}
