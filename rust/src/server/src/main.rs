mod message_type;
mod network_manager;

use crate::network_manager::NetworkManager;
use bevy::MinimalPlugins;
use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Query};

const SERVER_IP: &str = "127.0.0.1:3630";

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup_server)
        .add_systems(Update, update_server)
        .run();
}

fn setup_server(mut commands: Commands) {
    commands.spawn(NetworkManager::new(SERVER_IP));
}

fn update_server(network_manager: Query<&NetworkManager>) {
    for network_manager in network_manager.iter() {
        match network_manager.poll() {
            Ok(_) => {}
            Err(e) => println!("NetworkManager Error: {}", e),
        }
    }
}
