mod network;
mod replication;
mod input;

use crate::network::NetworkPlugin;
use crate::replication::ReplicationPlugin;
use crate::input::InputPlugin;
use bevy::MinimalPlugins;
use bevy::app::App;

const SERVER_IP: &str = "127.0.0.1:3630";
const SERVER_FREQUENCY: f64 = 30.0;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(NetworkPlugin)
        .add_plugins(ReplicationPlugin)
        .add_plugins(InputPlugin)
        .run();
}
