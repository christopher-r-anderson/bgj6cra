use bevy::{ecs::spawn::SpawnWith, input_focus::AutoFocus, prelude::*};
use bevy_flair::style::components::{ClassList, NodeStyleSheet};

use crate::{
    gameplay::level::{LevelState, LevelStats},
    menu::{NavigableChildren, button},
};

pub fn spawn_level_complete_menu(
    mut commands: Commands,
    asset_server: &AssetServer,
    level_stats: &LevelStats,
) {
    let heading = match (
        level_stats.success,
        level_stats.enemies_destroyed == level_stats.total_enemies,
    ) {
        (Some(true), _) => "Level Complete!",
        (Some(false), true) => "Almost!",
        (Some(false), false) => "Player Lost!",
        (None, _) => "Error: level stats not complete",
    };
    let text = match (
        level_stats.success,
        level_stats.enemies_destroyed == level_stats.total_enemies,
    ) {
        (Some(true), _) => "Good job! Keep going?",
        (Some(false), true) => {
            "You got the enemy, but their explosions got you too! You can do it!"
        }
        (Some(false), false) => "Nice try! Go again?",
        (None, _) => "Sorry, something went wrong!",
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
                                spawner.spawn(level_complete_success_menu(&level_stats));
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

fn level_complete_success_menu(_level_stats: &LevelStats) -> impl Bundle {
    (
        Name::new("Success Menu"),
        Node::default(),
        ClassList::new_with_classes(["dialog-menu"]),
        NavigableChildren::default(),
        Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
            spawner.spawn((button("Next Level"), AutoFocus));
            spawner.spawn(button("Replay Level"));
            spawner.spawn(button("Main Menu"));
        })),
    )
}

fn level_complete_failure_menu(_level_stats: &LevelStats) -> impl Bundle {
    (
        Name::new("Failure Menu"),
        Node::default(),
        ClassList::new_with_classes(["dialog-menu"]),
        NavigableChildren::default(),
        Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
            spawner.spawn(button("Replay Level"));
            spawner.spawn(button("Main Menu"));
        })),
    )
}

fn level_complete_error_menu() -> impl Bundle {
    (
        Name::new("Error Menu"),
        Node::default(),
        ClassList::new_with_classes(["dialog-menu"]),
        NavigableChildren::default(),
        Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
            spawner.spawn((button("Replay Level"), AutoFocus));
            spawner.spawn(button("Exit To Desktop"));
        })),
    )
}
