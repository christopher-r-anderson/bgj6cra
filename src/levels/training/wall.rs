use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Enemy Walls",
        notes: "You can't destroy Enemy Walls and your projectiles have no effect on them.\n\nFortunately, they don't explode.",
        start_position: vec2(0., -300.),
        enemies: vec![
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_wall(asset_server, vec2(0., 0.), vec2(200., 40.)),
        ],
    }
}
