use std::time::Duration;

use bevy::prelude::*;
use bevy_flair::style::components::{ClassList, NodeStyleSheet};

use crate::gameplay::level::LevelState;

pub struct ReadyScreenPlugin;

impl Plugin for ReadyScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Ready), spawn_countdown)
            .add_systems(Update, update_countdown.run_if(in_state(LevelState::Ready)));
    }
}

#[derive(Component, Debug)]
struct Countdown {
    count: u8,
    current: u8,
    timer: Timer,
}

impl Countdown {
    fn new(secs_per_count: f32, count: u8) -> Self {
        Self {
            timer: Timer::from_seconds(secs_per_count, TimerMode::Once),
            count,
            current: 0,
        }
    }
    fn is_complete(&self) -> bool {
        self.current >= self.count
    }
    fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            self.current += 1;
            self.timer.reset();
        }
    }
}

fn spawn_countdown(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            StateScoped(LevelState::Ready),
            Node::default(),
            ClassList::new_with_classes(["globally-center-children", "overlay", "countdown"]),
            NodeStyleSheet::new(asset_server.load("styles/all.css")),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new("Ready?"),
                ClassList::new_with_classes(["countdown-label"]),
            ));
            builder.spawn((
                Countdown::new(1., 3),
                Text::default(),
                ClassList::new_with_classes(["countdown-time"]),
            ));
        });
}

fn update_countdown(
    mut next_state: ResMut<NextState<LevelState>>,
    time: Res<Time>,
    mut countdown: Single<(&mut Countdown, &mut Text)>,
) {
    countdown.0.tick(time.delta());
    countdown.1.0 = format!("{:.0}", countdown.0.count - countdown.0.current);
    if countdown.0.is_complete() {
        next_state.set(LevelState::Playing);
    }
}
