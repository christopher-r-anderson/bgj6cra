use bevy::{ecs::spawn::SpawnWith, input_focus::AutoFocus, prelude::*};
use bevy_flair::style::components::{ClassList, NodeStyleSheet};

use crate::{
    app_state::AppState,
    gameplay::game_run::{GameRunMode, SelectedGameRunMode},
    menu::{ButtonActivate, NavigableChildren, button},
};

// Code here and in the related CSS file, originally from https://github.com/eckz/bevy_flair/blob/main/examples/game_menu.rs (MIT LICENSED)

pub fn spawn_main_menu(mut commands: Commands, asset_server: &AssetServer) {
    commands.spawn((
        StateScoped(AppState::Title),
        Name::new("Root"),
        Node::default(),
        ClassList::new_with_classes(["globally-center-children", "overlay"]),
        NodeStyleSheet::new(asset_server.load("styles/all.css")),
        children![
            (
                Node::default(),
                ClassList::new_with_classes(["column"]),
                children![
                    (
                        Text::new("Maxx"),
                        ClassList::new_with_classes(["title-maxx"]),
                        Transform::from_rotation(Quat::from_rotation_z(-90f32.to_radians())),
                    ),
                    (
                        Text::new("Obliterate"),
                        ClassList::new_with_classes(["title-obliterate"]),
                        Transform::from_rotation(Quat::from_rotation_z(90f32.to_radians())),
                    )
                ]
            ),
            (
                ClassList::new_with_classes(["game-menu"]),
                Node::default(),
                NavigableChildren::default(),
                Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
                    spawner.spawn((
                        ClassList::new_with_classes(["main-menu-heading"]),
                        Node::default(),
                        Children::spawn_one(Text::new("Main Menu")),
                    ));

                    spawner.spawn((button("Play"), AutoFocus)).observe(
                        |_trigger: Trigger<ButtonActivate>,
                         mut selected_mode: ResMut<SelectedGameRunMode>,
                         mut next_state: ResMut<NextState<AppState>>| {
                            selected_mode.0 = Some(GameRunMode::Game);
                            next_state.set(AppState::ResetGameRun);
                        },
                    );

                    spawner.spawn(button("Training")).observe(
                        |_trigger: Trigger<ButtonActivate>,
                         mut selected_mode: ResMut<SelectedGameRunMode>,
                         mut next_state: ResMut<NextState<AppState>>| {
                            selected_mode.0 = Some(GameRunMode::Training);
                            next_state.set(AppState::ResetGameRun);
                        },
                    );

                    spawner.spawn(button("Level Select")).observe(
                        |_trigger: Trigger<ButtonActivate>,
                         mut selected_mode: ResMut<SelectedGameRunMode>,
                         mut next_state: ResMut<NextState<AppState>>| {
                            selected_mode.0 = Some(GameRunMode::SingleLevel);
                            next_state.set(AppState::ResetGameRun);
                        },
                    );

                    spawner.spawn(button("Quit")).observe(
                        |_trigger: Trigger<ButtonActivate>,
                         mut exit_event: EventWriter<AppExit>| {
                            info!("Exiting");
                            exit_event.write_default();
                        },
                    );

                    // TODO: Reenable once layout is closer to finished
                    // spawner.spawn((
                    //     Name::new("floating_borders"),
                    //     Node::default(),
                    //     Pickable::IGNORE,
                    // ));
                })),
            )
        ],
    ));
}
