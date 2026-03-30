use crate::replication::replicated_nodes::player::Player;
use crate::replication::replication_manager::ReplicationManager;
use bevy::prelude::{Event, On, Query, ResMut};
use common::input_packet::InputPacket;
use common::stream_reader::StreamReader;

#[derive(Event)]
pub struct InputReceived {
    pub stream_reader: StreamReader,
}

pub fn handle_input(
    mut on_rpc_received: On<InputReceived>,
    mut players: Query<&mut Player>,
    mut replication_manager: ResMut<ReplicationManager>,
) {
    let node_id = on_rpc_received.stream_reader.read_u32();
    let input_packets: Vec<InputPacket> = on_rpc_received.stream_reader.read_serializable_vec();

    if let Some(mut player) = players.iter_mut().find(|node| node.net_id == node_id) {
        if let Some(client) = replication_manager
            .client_entities
            .get_mut(&player.owner_id)
        {
            for input_packet in input_packets {
                if input_packet.sequence <= client.last_sequence || input_packet.keys == 0 {
                    continue;
                }

                client.last_sequence = input_packet.sequence;

                player.handle_input(input_packet);
            }
        }
    }
}
