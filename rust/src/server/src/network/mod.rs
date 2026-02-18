use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Query, Res, ResMut};
use crate::network::network_manager::NetworkManager;
use crate::replication::replicated_node::ReplicatedNode;
use crate::replication::replication_manager::ReplicationManager;
use crate::rpc::rpc_manager::RpcManager;
use crate::SERVER_IP;

pub mod connected_client;
pub mod network_manager;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkManager::new(SERVER_IP))
            .add_systems(Update, poll);
    }
}

fn poll(
    commands: Commands,
    network_manager: Res<NetworkManager>,
    mut replication_manager: ResMut<ReplicationManager>,
    rpc_manager: ResMut<RpcManager>,
    replicated_nodes: Query<&mut ReplicatedNode>
) {
    match network_manager.poll(commands, replication_manager.as_mut()) {
        Some((client_addr, message_header, buffer)) => {
            rpc_manager.handle_rpc(
                client_addr,
                message_header,
                buffer,
                replicated_nodes
            );
        }
        None => {}
    }
}