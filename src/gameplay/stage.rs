use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_flair::style::components::{ClassList, NodeStyleSheet};

use crate::{
    app_state::AppState,
    gameplay::{
        game_run::GameRun,
        level::{LevelConfig, LevelStats},
    },
};

pub const STAGE_WIDTH: f32 = 520.;
pub const STAGE_HEIGHT: f32 = 720.;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_level_stopwatch_text);
    }
}

fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 1000.0;

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: STAGE_HEIGHT,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 0., 100.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn spawn_stage(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        // TODO: specify scope outside of file to reduce coupling
        StateScoped(AppState::Gameplay),
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("backgrounds/generic-background.glb")),
        ),
    ));
}

pub fn spawn_level_info_panel(
    commands: &mut Commands,
    asset_server: &AssetServer,
    level_config: &LevelConfig,
    game_run: &GameRun,
) -> impl Bundle {
    let LevelConfig { name, notes, .. } = level_config;
    commands.spawn((
        StateScoped(AppState::Gameplay),
        NodeStyleSheet::new(asset_server.load("styles/all.css")),
        Node::default(),
        ClassList::new_with_classes(["level-info-panel"]),
        Children::spawn_one((
            Node::default(),
            ClassList::new_with_classes(["level-info-panel-content"]),
            children![
                (Text::new(*notes), ClassList::new_with_classes(["text"])),
                (
                    Text::new(format!(
                        "{:02}/{:02}",
                        game_run.current_level_index() + 1,
                        game_run.total_level_count()
                    )),
                    ClassList::new_with_classes(["heading"])
                ),
                (Text::new(*name), ClassList::new_with_classes(["heading"])),
            ],
        )),
    ));
}

#[derive(Component, Debug)]
struct LevelStopwatchText;

pub fn spawn_level_stats_panel(commands: &mut Commands, asset_server: &AssetServer) -> impl Bundle {
    commands.spawn((
        StateScoped(AppState::Gameplay),
        NodeStyleSheet::new(asset_server.load("styles/all.css")),
        Node::default(),
        ClassList::new_with_classes(["level-stats-panel"]),
        Children::spawn_one((
            Text::default(),
            Children::spawn_one((TextSpan::new(format!("{:.2}", 0.)), LevelStopwatchText)),
        )),
    ));
}

fn update_level_stopwatch_text(
    level_stats: Single<&LevelStats>,
    mut span_q: Query<&mut TextSpan, With<LevelStopwatchText>>,
) {
    for mut span in &mut span_q {
        **span = format!("{:.2}", level_stats.stopwatch.elapsed_secs());
    }
}
