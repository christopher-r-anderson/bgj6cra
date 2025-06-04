use bevy::prelude::*;

use crate::{gameplay::enemy::EnemyBundle, screen::Screen};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelState>()
            .register_type::<Level>()
            .add_systems(
                FixedUpdate,
                check_load_status.run_if(in_state(LevelState::Loading)),
            );
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Level;

pub struct LevelConfig {
    pub start_position: Vec2,
    pub enemies: Vec<EnemyBundle>,
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum LevelState {
    #[default]
    Loading,
    Ready,
    Playing,
    Complete,
}

fn check_load_status(mut next_state: ResMut<NextState<LevelState>>) {
    next_state.set(LevelState::Playing);
}
