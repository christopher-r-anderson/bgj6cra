use bevy::prelude::*;

use crate::gameplay::enemy::EnemyBundle;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Level>();
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Level;

pub struct LevelConfig {
    pub start_position: Vec2,
    pub enemies: Vec<EnemyBundle>,
}
