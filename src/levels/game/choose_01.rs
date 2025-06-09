use bevy::prelude::*;

use crate::gameplay::{
    enemy::{DefenderClass, EnemyBundle},
    level::LevelConfig,
};

pub fn get_config(asset_server: &AssetServer) -> LevelConfig {
    let square_offsets = &[
        vec2(-15., 15.),
        vec2(15., 15.),
        vec2(15., -15.),
        vec2(-15., -15.),
    ];
    let line_offsets = &[
        vec2(0., -30. * 1.),
        vec2(0., -30. * 2.),
        vec2(0., -30. * 3.),
        vec2(0., -30. * 4.),
        vec2(0., -30. * 5.),
        vec2(0., -30. * 6.),
        vec2(0., -30. * 7.),
    ];

    let mut enemies = vec![EnemyBundle::new_base(asset_server, vec2(0., 330.))];

    enemies.extend(group(
        asset_server,
        vec2(-105., 245.),
        DefenderClass::One,
        line_offsets,
    ));

    enemies.extend(group(
        asset_server,
        vec2(-60., 200.),
        DefenderClass::One,
        square_offsets,
    ));
    enemies.extend(group(
        asset_server,
        vec2(0., 200.),
        DefenderClass::Two,
        square_offsets,
    ));
    enemies.extend(group(
        asset_server,
        vec2(60., 200.),
        DefenderClass::Three,
        square_offsets,
    ));

    enemies.extend(group(
        asset_server,
        vec2(-60., 50.),
        DefenderClass::One,
        square_offsets,
    ));
    enemies.extend(group(
        asset_server,
        vec2(0., 50.),
        DefenderClass::Two,
        square_offsets,
    ));
    enemies.extend(group(
        asset_server,
        vec2(60., 50.),
        DefenderClass::Three,
        square_offsets,
    ));

    enemies.extend(group(
        asset_server,
        vec2(105., 245.),
        DefenderClass::Three,
        line_offsets,
    ));

    LevelConfig {
        name: "Choose Your Destiny",
        notes: "",
        start_position: vec2(0., 100.),
        enemies,
    }
}

fn group(
    asset_server: &AssetServer,
    pos: Vec2,
    class: DefenderClass,
    offsets: &[Vec2],
) -> Vec<EnemyBundle> {
    offsets
        .iter()
        .map(|offset| EnemyBundle::new_defender(asset_server, pos + offset, &class))
        .collect()
}
