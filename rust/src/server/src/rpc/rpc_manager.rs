use crate::replication::replicated_node::ReplicatedNode;
use crate::replication::replication_manager::ReplicationManager;
use bevy::prelude::{Event, On, Query, ResMut};
use common::input_packet::{Input, InputPacket};
use common::stream_reader::StreamReader;

#[derive(Event)]
pub struct RpcReceived {
    pub stream_reader: StreamReader,
}

pub fn handle_rpc(
    mut on_rpc_received: On<RpcReceived>,
    mut replicated_nodes: Query<&mut ReplicatedNode>,
    mut replication_manager: ResMut<ReplicationManager>,
) {
    let node_id = on_rpc_received.stream_reader.read_u32();
    let input_packets: Vec<InputPacket> = on_rpc_received.stream_reader.read_serializable_vec();

    if let Some(mut replicated_node) = replicated_nodes
        .iter_mut()
        .find(|node| node.net_id == node_id)
    {
        if let Some(client) = replication_manager
            .client_entities
            .get_mut(&replicated_node.owner_id)
        {
            for input_packet in input_packets {
                if input_packet.sequence <= client.last_sequence {
                    continue;
                }

                let x = if input_packet.read_input(Input::Right) {
                    5.0
                } else if input_packet.read_input(Input::Left) {
                    -5.0
                } else {
                    0.0
                };
                replicated_node.x += x;
                let y = if input_packet.read_input(Input::Down) {
                    5.0
                } else if input_packet.read_input(Input::Up) {
                    -5.0
                } else {
                    0.0
                };
                replicated_node.y += y;

                client.last_sequence = input_packet.sequence;
            }
        }
    }
}
