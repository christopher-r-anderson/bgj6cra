use bevy::{ecs::spawn::SpawnWith, input_focus::AutoFocus, prelude::*};
use bevy_flair::style::components::NodeStyleSheet;

use crate::{
    menu::{ButtonActivate, NavigableChildren, button},
    screen::Screen,
};

pub fn spawn_menu(mut commands: Commands, asset_server: &AssetServer) {
    commands.spawn((
        StateScoped(Screen::Title),
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

                spawner.spawn((button("Training"), AutoFocus)).observe(
                    |_trigger: Trigger<ButtonActivate>,
                     mut next_state: ResMut<NextState<Screen>>| {
                        info!("Clicked");
                        next_state.set(Screen::Gameplay);
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
