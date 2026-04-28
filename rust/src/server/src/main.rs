mod input;
mod network;
mod replication;

use crate::input::InputPlugin;
use crate::network::NetworkPlugin;
use crate::replication::ReplicationPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, ScheduleRunnerPlugin};
use bevy::prelude::default;
use bevy::window::WindowPlugin;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

const SERVER_IP: &str = "127.0.0.1:3630";
const SERVER_FREQUENCY: f64 = 60.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    ..default()
                })
                .disable::<bevy::winit::WinitPlugin>(),
        )
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / SERVER_FREQUENCY,
        )))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(NetworkPlugin)
        .add_plugins(ReplicationPlugin)
        .add_plugins(InputPlugin)
        .run();
}
