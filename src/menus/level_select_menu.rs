use bevy::{ecs::spawn::SpawnWith, input_focus::AutoFocus, prelude::*};
use bevy_flair::style::components::{ClassList, NodeStyleSheet};

use crate::{
    app_state::AppState,
    gameplay::game_run::{GameRun, GameRunMode, SelectedGameRunMode},
    menu::{ButtonActivate, NavigableChildren, button},
};

pub fn spawn_level_select_menu(mut commands: Commands, asset_server: &AssetServer) {
    let level_configs = GameRun::game_levels()
        .into_iter()
        .map(|get_config| (get_config, get_config(asset_server)))
        .collect::<Vec<_>>();
    commands.spawn((
        StateScoped(AppState::GameRun),
        ClassList::new_with_classes(["globally-center-children", "overlay"]),
        NodeStyleSheet::new(asset_server.load("styles/base.css")),
        Node::default(),
        Children::spawn_one((
            Node::default(),
            ClassList::new_with_classes(["column"]),
            children![
                (
                    Text::new("Select a Level"),
                    ClassList::new_with_classes(["heading"])
                ),
                (
                    Name::new("Level Select Menu"),
                    ClassList::new_with_classes(["dialog-menu"]),
                    Node::default(),
                    NavigableChildren::default(),
                    Children::spawn(SpawnWith(move |spawner: &mut ChildSpawner| {
                        level_configs.into_iter().enumerate().for_each(
                            |(index, (get_config, level_config))| {
                                let mut button = spawner.spawn(button(level_config.name));
                                button.observe(
                                        move |
                                            _trigger: Trigger<ButtonActivate>,
                                            mut commands: Commands,
                                            mut selected_mode: ResMut<SelectedGameRunMode>,
                                            mut next_state: ResMut<NextState<AppState>>
                                        | {
                                            selected_mode.0 = Some(GameRunMode::SingleLevel);
                                            commands.spawn(GameRun::new_single_level(get_config));
                                            next_state.set(AppState::ResetGameplay);
                                        },
                                    );
                                if index == 0 {
                                    button.insert(AutoFocus);
                                }
                            },
                        );
                    })),
                ),
            ],
        )),
    ));
}
