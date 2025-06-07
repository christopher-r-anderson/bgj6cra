use bevy::{ecs::spawn::SpawnWith, input_focus::AutoFocus, prelude::*};
use bevy_flair::style::components::{ClassList, NodeStyleSheet};

use crate::{
    app_state::AppState,
    gameplay::{
        game_run::{GameRun, GameRunMode},
        level::{LevelState, LevelStats},
    },
    menu::{ButtonActivate, NavigableChildren, button},
};

#[derive(Clone, Debug)]
enum LevelCompletionStatus {
    Error,
    LostEnemiesDestroyed,
    LostEnemiesRemaining,
    Survived,
}

impl From<&LevelStats> for LevelCompletionStatus {
    fn from(level_stats: &LevelStats) -> Self {
        match (
            level_stats.success,
            level_stats.enemies_destroyed == level_stats.total_enemies,
        ) {
            (Some(true), _) => LevelCompletionStatus::Survived,
            (None, _) => LevelCompletionStatus::Error,
            (_, true) => LevelCompletionStatus::LostEnemiesDestroyed,
            (_, false) => LevelCompletionStatus::LostEnemiesRemaining,
        }
    }
}

#[derive(Clone, Debug)]
enum NextLevelStatus {
    MoreLevels,
    GameComplete,
    SingleLevelRun,
}

impl From<&GameRun> for NextLevelStatus {
    fn from(game_run: &GameRun) -> Self {
        match (game_run.mode(), game_run.has_more_levels()) {
            (GameRunMode::SingleLevel, _) => NextLevelStatus::SingleLevelRun,
            (_, true) => NextLevelStatus::MoreLevels,
            (_, false) => NextLevelStatus::GameComplete,
        }
    }
}

pub fn spawn_level_complete_menu(
    mut commands: Commands,
    asset_server: &AssetServer,
    level_stats: &LevelStats,
    game_run: &GameRun,
) {
    let next_level_status = NextLevelStatus::from(game_run);
    let level_completion_status = LevelCompletionStatus::from(level_stats);
    let (heading, text) = match (&level_completion_status, &next_level_status) {
        (LevelCompletionStatus::Survived, NextLevelStatus::MoreLevels) => {
            ("Level Complete!", "Good job! Keep going?")
        }
        (LevelCompletionStatus::Survived, NextLevelStatus::SingleLevelRun) => {
            ("Level Complete!", "Good job! Replay?")
        }
        (LevelCompletionStatus::Survived, NextLevelStatus::GameComplete) => {
            ("Congratulations!!!", "You beat the game! Play again?")
        }

        (LevelCompletionStatus::LostEnemiesDestroyed, _) => (
            "Almost!",
            "You got the enemy, but their explosions got you too! You can do it!",
        ),
        (LevelCompletionStatus::LostEnemiesRemaining, _) => ("Player Lost!", "Nice try! Go again?"),

        (LevelCompletionStatus::Error, _) => (
            "Error: level stats not complete",
            "Sorry, something went wrong!",
        ),
    };
    let stats = format!(
        "Level Enemies Destroyed: {}/{}",
        level_stats.enemies_destroyed, level_stats.total_enemies
    );
    let level_stats = level_stats.clone();
    let has_more_levels = game_run.has_more_levels();
    commands.spawn((
        StateScoped(LevelState::Complete),
        ClassList::new_with_classes([
            "level-complete-dialog",
            "globally-center-children",
            "overlay",
        ]),
        NodeStyleSheet::new(asset_server.load("styles/all.css")),
        Node::default(),
        Children::spawn_one((
            Node::default(),
            ClassList::new_with_classes(["level-complete-dialog"]),
            children![
                (Text::new(heading), ClassList::new_with_classes(["heading"])),
                (Text::new(text), ClassList::new_with_classes(["text"])),
                (Text::new(stats), ClassList::new_with_classes(["text"])),
                (
                    Name::new("Level Complete Dialog Menu"),
                    Node::default(),
                    Children::spawn(SpawnWith(move |spawner: &mut ChildSpawner| {
                        match (level_stats.success, next_level_status) {
                            (Some(true), NextLevelStatus::MoreLevels) => {
                                spawner.spawn(level_complete_success_menu(
                                    &level_stats,
                                    has_more_levels,
                                ));
                            }
                            _ => {
                                spawner.spawn(no_advancement_level(&level_stats));
                            }
                        }
                    })),
                ),
            ],
        )),
    ));
}

fn level_complete_success_menu(_level_stats: &LevelStats, has_more_levels: bool) -> impl Bundle {
    (
        Name::new("Level Success Menu"),
        Node::default(),
        ClassList::new_with_classes(["dialog-menu"]),
        NavigableChildren::default(),
        Children::spawn(SpawnWith(move |spawner: &mut ChildSpawner| {
            if has_more_levels {
                spawner.spawn((button("Next Level"), AutoFocus)).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut next_state: ResMut<NextState<AppState>>,
                     mut game_run: Single<&mut GameRun>| {
                        if let Err(message) = game_run.advance_current_level() {
                            // TODO: Show this in the ui
                            warn!(message);
                        } else {
                            next_state.set(AppState::ResetGameplay);
                        }
                    },
                );
                spawner.spawn(button("Replay Level")).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::ResetGameplay);
                    },
                );
                spawner.spawn(button("Main Menu")).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::Title);
                    },
                );
            } else {
                spawner.spawn((button("Main Menu"), AutoFocus)).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::Title);
                    },
                );
                spawner.spawn(button("Replay Level")).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::ResetGameplay);
                    },
                );
            }
        })),
    )
}

fn no_advancement_level(_level_stats: &LevelStats) -> impl Bundle {
    (
        Name::new("No Advancement Menu"),
        Node::default(),
        ClassList::new_with_classes(["dialog-menu"]),
        NavigableChildren::default(),
        // TODO: allow moving ahead if the level has previously been beaten?
        Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
            spawner.spawn((button("Replay Level"), AutoFocus)).observe(
                |_trigger: Trigger<ButtonActivate>, mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::ResetGameplay);
                },
            );
            spawner.spawn(button("Main Menu")).observe(
                |_trigger: Trigger<ButtonActivate>, mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::Title);
                },
            );
        })),
    )
}
