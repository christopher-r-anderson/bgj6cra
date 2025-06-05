use avian2d::prelude::*;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_skein::SkeinPlugin;
#[cfg(debug_assertions)]
use maxx_obliterate::debug::DebugPlugin;
use maxx_obliterate::{
    gameplay::{
        collisions::CollisionPlugin, enemy::EnemyPlugin, energy::EnergyPlugin,
        explosion::ExplosionPlugin, level::LevelPlugin, player::PlayerPlugin, stage::StagePlugin,
    },
    menu::MenuPlugin,
    screen::ScreenPlugin,
    window::AppWindowPlugin,
};

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "maxx obliterate - Bevy Game Jam 6".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );
        app.add_plugins(SkeinPlugin::default());
        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity(Vec2::ZERO));
        app.add_plugins((
            AppWindowPlugin,
            CollisionPlugin,
            #[cfg(debug_assertions)]
            DebugPlugin,
            EnemyPlugin,
            EnergyPlugin,
            ExplosionPlugin,
            LevelPlugin,
            MenuPlugin,
            PlayerPlugin,
            ScreenPlugin,
            StagePlugin,
        ));
    }
}
