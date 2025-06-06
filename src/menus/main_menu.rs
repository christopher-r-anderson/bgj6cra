use bevy::{ecs::spawn::SpawnWith, input_focus::AutoFocus, prelude::*};
use bevy_flair::style::components::NodeStyleSheet;

use crate::{
    app_state::AppState,
    gameplay::game_run::SelectedGameRunMode,
    menu::{ButtonActivate, NavigableChildren, button},
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: &AssetServer) {
    commands.spawn((
        StateScoped(AppState::Title),
        Name::new("Root"),
        Node::default(),
        NodeStyleSheet::new(asset_server.load("styles/game_menu.css")),
        children![(
            Name::new("game_menu"),
            Node::default(),
            NavigableChildren::default(),
            Children::spawn(SpawnWith(|spawner: &mut ChildSpawner| {
                spawner.spawn((
                    Name::new("menu_title"),
                    Node::default(),
                    Children::spawn_one(Text::new("Main Menu")),
                ));

                spawner.spawn((button("Play"), AutoFocus)).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut selected_mode: ResMut<SelectedGameRunMode>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        *selected_mode = SelectedGameRunMode::Game;
                        next_state.set(AppState::ResetGameRun);
                    },
                );

                spawner.spawn(button("Training")).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut selected_mode: ResMut<SelectedGameRunMode>,
                     mut next_state: ResMut<NextState<AppState>>| {
                        *selected_mode = SelectedGameRunMode::Training;
                        next_state.set(AppState::ResetGameRun);
                    },
                );

                spawner.spawn(button("Quit")).observe(
                    |_trigger: Trigger<ButtonActivate>, mut exit_event: EventWriter<AppExit>| {
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
        )],
    ));
}
