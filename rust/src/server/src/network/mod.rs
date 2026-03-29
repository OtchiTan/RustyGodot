use crate::SERVER_IP;
use crate::network::network_manager::NetworkManager;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

pub mod connected_client;
pub mod network_manager;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManager::new(SERVER_IP))
            .add_systems(Update, poll);
    }
}

fn poll(commands: Commands, network_manager: Res<NetworkManager>) {
    network_manager.poll(commands);
}
