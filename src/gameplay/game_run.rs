use bevy::prelude::*;

use crate::{
    gameplay::level::LevelConfig,
    levels::{training_01, training_02},
};

pub struct GameRunPlugin;

impl Plugin for GameRunPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LevelStatus {
    #[default]
    Unplayed,
    Tried,
    Completed,
}

#[derive(Component, Clone, Debug)]
pub struct GameRun {
    index: usize,
    level_statuses: [LevelStatus; 2],
    levels: [fn(&AssetServer) -> LevelConfig; 2],
}

impl GameRun {
    pub fn new_training() -> Self {
        Self {
            index: 0,
            level_statuses: [LevelStatus::default(); 2],
            levels: [training_01::get_config, training_02::get_config],
        }
    }
    pub fn advance_current_level(&mut self) -> Result<(), String> {
        if self.has_more_levels() {
            self.index += 1;
            Ok(())
        } else {
            Err("No more levels to advance to.".into())
        }
    }
    pub fn current_level_config(&self, asset_server: &AssetServer) -> LevelConfig {
        self.levels[self.index](asset_server)
    }
    pub fn has_more_levels(&self) -> bool {
        self.index + 1 < self.levels.len()
    }
    pub fn set_current_level_status(&mut self, level_status: LevelStatus) {
        self.level_statuses[self.index] = level_status;
    }
    pub fn current_level_index(&self) -> usize {
        self.index
    }
    pub fn total_level_count(&self) -> usize {
        self.levels.len()
    }
}
