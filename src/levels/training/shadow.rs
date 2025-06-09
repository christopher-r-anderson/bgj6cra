use bevy::prelude::*;

use crate::gameplay::{enemy::EnemyBundle, level::LevelConfig};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    LevelConfig {
        name: "Shadow",
        notes: "Outlined Defenders are invulnerable to projectiles, though you can still shoot past them.\n\nThey will explode before Defenders.",
        start_position: vec2(0., 0.),
        enemies: vec![
            EnemyBundle::new_shadow(asset_server, vec2(-80., 330.)),
            EnemyBundle::new_shadow(asset_server, vec2(80., 330.)),
            EnemyBundle::new_base(asset_server, vec2(0., 330.)),
            EnemyBundle::new_shadow(asset_server, vec2(-80., 290.)),
            EnemyBundle::new_shadow(asset_server, vec2(-40., 290.)),
            EnemyBundle::new_shadow(asset_server, vec2(0., 290.)),
            EnemyBundle::new_shadow(asset_server, vec2(40., 290.)),
            EnemyBundle::new_shadow(asset_server, vec2(80., 290.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(-100., 200.)),
            EnemyBundle::new_primary_defender(asset_server, vec2(100., 200.)),
        ],
    }
}
