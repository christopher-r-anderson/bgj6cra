use bevy::prelude::*;

use crate::{
    gameplay::{
        enemy::{Enemy, EnemyBundle, EnemyDestroyedEvent},
        explosion::Explosion,
        player::{PlayerDestroyedEvent, player_bundle},
        stage::spawn_stage,
    },
    levels::training_01,
    screen::Screen,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LevelState>()
            .register_type::<Level>()
            .add_observer(on_enemy_destroyed)
            .add_observer(on_player_destroyed)
            .add_systems(
                FixedUpdate,
                check_level_complete.run_if(in_state(LevelState::Playing)),
            )
            .add_systems(OnEnter(LevelState::Loading), spawn_level)
            .add_systems(OnEnter(LevelState::Complete), show_level_complete_message)
            .add_systems(
                FixedUpdate,
                check_load_status.run_if(in_state(LevelState::Loading)),
            );
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Level;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct LevelStats {
    enemies_destroyed: u32,
    success: Option<bool>,
    total_enemies: u32,
}

impl LevelStats {
    fn new(total_enemies: u32) -> Self {
        Self {
            enemies_destroyed: 0,
            success: None,
            total_enemies,
        }
    }
}

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

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_config = training_01::get_config(&asset_server);
    commands.spawn((
        StateScoped(Screen::Gameplay),
        LevelStats::new(
            level_config
                .enemies
                .len()
                .try_into()
                .expect("There shouldn't be more enemies than u32"),
        ),
    ));
    commands.spawn_batch(level_config.enemies);
    commands.spawn((
        StateScoped(Screen::Gameplay),
        player_bundle(&asset_server, level_config.start_position),
    ));
    spawn_stage(commands, &asset_server);
}

fn check_load_status(mut next_state: ResMut<NextState<LevelState>>) {
    next_state.set(LevelState::Playing);
}

fn on_enemy_destroyed(
    _trigger: Trigger<EnemyDestroyedEvent>,
    mut level_stats: Single<&mut LevelStats>,
) {
    // FIXME: count is x2, possibly because of duplicate destroyed events. check collisions and chain.
    level_stats.enemies_destroyed += 1;
}

fn check_level_complete(
    mut next_state: ResMut<NextState<LevelState>>,
    mut level_stats: Single<&mut LevelStats>,
    enemies_q: Query<(), With<Enemy>>,
    explosions_q: Query<(), With<Explosion>>,
) {
    if enemies_q.is_empty() && explosions_q.is_empty() {
        level_stats.success = Some(true);
        next_state.set(LevelState::Complete);
    }
}

fn on_player_destroyed(
    _trigger: Trigger<PlayerDestroyedEvent>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut level_stats: Single<&mut LevelStats>,
) {
    level_stats.success = Some(false);
    next_state.set(LevelState::Complete);
}

fn show_level_complete_message(level_stats: Single<&LevelStats>) {
    info!("{:?}", level_stats.into_inner());
}
