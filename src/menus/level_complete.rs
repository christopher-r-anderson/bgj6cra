use bevy::{ecs::spawn::SpawnWith, input_focus::AutoFocus, prelude::*};
use bevy_flair::style::components::{ClassList, NodeStyleSheet};

use crate::{
    app_state::AppState,
    gameplay::{
        game_run::GameRun,
        level::{LevelState, LevelStats},
    },
    menu::{ButtonActivate, NavigableChildren, button},
};

pub fn spawn_level_complete_menu(
    mut commands: Commands,
    asset_server: &AssetServer,
    level_stats: &LevelStats,
    game_run: &GameRun,
) {
    let has_more_levels = game_run.has_more_levels();
    let heading = match (
        level_stats.success,
        level_stats.enemies_destroyed == level_stats.total_enemies,
        game_run.has_more_levels(),
    ) {
        (Some(true), _, true) => "Level Complete!",
        (Some(true), _, false) => "Congratulations!!!",
        (Some(false), true, _) => "Almost!",
        (Some(false), false, _) => "Player Lost!",
        (None, _, _) => "Error: level stats not complete",
    };
    let text = match (
        level_stats.success,
        level_stats.enemies_destroyed == level_stats.total_enemies,
        game_run.has_more_levels(),
    ) {
        (Some(true), _, true) => "Good job! Keep going?",
        (Some(true), _, false) => "You beat the game! Play again?",
        (Some(false), true, _) => {
            "You got the enemy, but their explosions got you too! You can do it!"
        }
        (Some(false), false, _) => "Nice try! Go again?",
        (None, _, _) => "Sorry, something went wrong!",
    };
    let stats = format!(
        "Enemies: {}/{}",
        level_stats.enemies_destroyed, level_stats.total_enemies
    );
    let level_stats = level_stats.clone();
    commands.spawn((
        StateScoped(LevelState::Complete),
        ClassList::new_with_classes([
            "level-complete-dialog",
            "globally-center-children",
            "overlay",
        ]),
        NodeStyleSheet::new(asset_server.load("styles/level_complete.css")),
        Node::default(),
        Children::spawn_one((
            Node::default(),
            ClassList::new_with_classes(["level-complete-dialog"]),
            children![
                (
                    Text::new(heading),
                    ClassList::new_with_classes(["dialog-heading"])
                ),
                (
                    Text::new(text),
                    ClassList::new_with_classes(["dialog-text"])
                ),
                (
                    Text::new(stats),
                    ClassList::new_with_classes(["dialog-text"])
                ),
                (
                    Name::new("Level Complete Dialog Menu"),
                    Node::default(),
                    Children::spawn(SpawnWith(move |spawner: &mut ChildSpawner| {
                        match level_stats.success {
                            Some(true) => {
                                spawner.spawn(level_complete_success_menu(
                                    &level_stats,
                                    has_more_levels,
                                ));
                            }
                            Some(false) => {
                                spawner.spawn(level_complete_failure_menu(&level_stats));
                            }
                            None => {
                                spawner.spawn(level_complete_error_menu());
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
            } else {
                spawner.spawn((button("Replay Level"), AutoFocus)).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::ResetGameplay);
                    },
                );
            }
            spawner.spawn(button("Main Menu")).observe(
                |_trigger: Trigger<ButtonActivate>, mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::Title);
                },
            );
        })),
    )
}

fn level_complete_failure_menu(_level_stats: &LevelStats) -> impl Bundle {
    (
        Name::new("Level Failure Menu"),
        Node::default(),
        ClassList::new_with_classes(["dialog-menu"]),
        NavigableChildren::default(),
        // TODO: allow moving ahead if the level has previously been beaten?
        Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
            spawner.spawn(button("Replay Level")).observe(
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

fn level_complete_error_menu() -> impl Bundle {
    (
        Name::new("Level Error Menu"),
        Node::default(),
        ClassList::new_with_classes(["dialog-menu"]),
        NavigableChildren::default(),
        Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
            spawner.spawn((button("Replay Level"), AutoFocus)).observe(
                |_trigger: Trigger<ButtonActivate>, mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::ResetGameplay);
                },
            );
            spawner.spawn(button("Exit To Desktop"));
        })),
    )
}
