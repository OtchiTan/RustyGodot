mod connected_client;
mod message_header;
mod network_manager;
mod network_mapping;
mod replicated_node;
mod serializer;

use crate::connected_client::ConnectedClient;
use crate::message_header::{DataType, MessageHeader, MessageType};
use crate::network_manager::NetworkManager;
use crate::network_mapping::NetworkMapping;
use crate::replicated_node::ReplicatedNode;
use crate::serializer::Serializer;
use bevy::MinimalPlugins;
use bevy::app::{App, FixedUpdate, Update};
use bevy::prelude::*;
use bevy::time::{Fixed, Time};
use std::collections::HashMap;

const SERVER_IP: &str = "127.0.0.1:3630";

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(NetworkManager::new(SERVER_IP))
        .insert_resource(NetworkMapping {
            map: HashMap::new(),
        })
        .insert_resource(Time::<Fixed>::from_hz(30.0))
        .add_systems(FixedUpdate, update_replication)
        .add_systems(Update, poll)
        .run();
}

fn poll(
    commands: Commands,
    network_manager: Res<NetworkManager>,
    network_mapping: ResMut<NetworkMapping>,
) {
    match network_manager.poll(commands, network_mapping) {
        Ok(_) => {}
        Err(e) => println!("NetworkManager Error: {}", e),
    }
}

fn update_replication(
    network_manager: Res<NetworkManager>,
    clients: Query<&ConnectedClient>,
    replicated_nodes: Query<&ReplicatedNode>,
) {
    for replicated_node in replicated_nodes.iter() {
        let message_header = MessageHeader::new(MessageType::Data, DataType::Replication);
        let mut serializer = Serializer::new(vec![]);
        let _ = &mut serializer << message_header.get_data();
        let _ = &mut serializer << replicated_node.net_id;
        let _ = &mut serializer << replicated_node.class_id;
        let _ = &mut serializer << replicated_node.x;
        let _ = &mut serializer << replicated_node.y;

        for client in clients.iter() {
            network_manager.send_data(&client.address, serializer.get_data());
        }
    }
}
