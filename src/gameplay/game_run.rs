use bevy::prelude::*;

use crate::{
    gameplay::level::LevelConfig,
    levels::{
        game::{game_01, game_02, in_yer_face_1, in_yer_face_2, in_yer_face_3, path},
        training::{training_01, training_02, training_03, wall},
    },
};

pub struct GameRunPlugin;

impl Plugin for GameRunPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedGameRunMode>();
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LevelStatus {
    #[default]
    Unplayed,
    Tried,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameRunMode {
    Training,
    Game,
    SingleLevel,
}

#[derive(Resource, Debug, Default)]
pub struct SelectedGameRunMode(pub Option<GameRunMode>);

#[derive(Component, Clone, Debug)]
pub struct GameRun {
    index: usize,
    level_statuses: Vec<LevelStatus>,
    // TODO: instead of passing around config methods to avoid useless resource usage, instantiate enemies on demand
    levels: Vec<fn(&AssetServer) -> LevelConfig>,
    mode: GameRunMode,
}

impl GameRun {
    pub fn game_levels() -> Vec<fn(&AssetServer) -> LevelConfig> {
        vec![
            in_yer_face_1::get_config,
            in_yer_face_2::get_config,
            in_yer_face_3::get_config,
            path::get_config,
            game_01::get_config,
            game_02::get_config,
        ]
    }
    pub fn training_levels() -> Vec<fn(&AssetServer) -> LevelConfig> {
        vec![
            wall::get_config,
            training_01::get_config,
            training_02::get_config,
            training_03::get_config,
        ]
    }
    pub fn new_game() -> Self {
        let levels = Self::game_levels();
        Self {
            index: 0,
            level_statuses: vec![LevelStatus::default(); levels.len()],
            levels,
            mode: GameRunMode::Game,
        }
    }
    pub fn new_training() -> Self {
        let levels = Self::training_levels();
        Self {
            index: 0,
            level_statuses: vec![LevelStatus::default(); levels.len()],
            levels,
            mode: GameRunMode::Training,
        }
    }
    pub fn new_single_level(level_get_config: fn(&AssetServer) -> LevelConfig) -> Self {
        Self {
            index: 0,
            level_statuses: vec![LevelStatus::default(); 1],
            levels: vec![level_get_config],
            mode: GameRunMode::SingleLevel,
        }
    }
    pub fn mode(&self) -> GameRunMode {
        self.mode
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
