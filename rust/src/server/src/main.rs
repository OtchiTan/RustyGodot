mod network;
mod replication;
mod rpc;

use crate::replication::replicated_node::ReplicatedNode;
use crate::replication::replication_manager::ReplicationManager;
use crate::rpc::rpc_manager::RpcManager;
use bevy::MinimalPlugins;
use bevy::app::{App, FixedUpdate, Update};
use bevy::prelude::*;
use bevy::time::{Fixed, Time};
use common::message_header::{DataType, MessageHeader, MessageType};
use common::serializer::Serializer;
use network::connected_client::ConnectedClient;
use network::network_manager::NetworkManager;
use std::collections::HashMap;

const SERVER_IP: &str = "127.0.0.1:3630";

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(NetworkManager::new(SERVER_IP))
        .insert_resource(RpcManager {})
        .insert_resource(ReplicationManager {
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
    mut replication_manager: ResMut<ReplicationManager>,
    rpc_manager: ResMut<RpcManager>,
) {
    match network_manager.poll(commands, replication_manager.as_mut()) {
        Some((client_addr, message_header, buffer)) => {
            rpc_manager.handle_rpc(
                client_addr,
                message_header,
                buffer,
                replication_manager.as_ref(),
            );
        }
        None => {}
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
