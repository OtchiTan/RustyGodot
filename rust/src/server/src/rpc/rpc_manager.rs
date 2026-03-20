use crate::replication::replicated_node::ReplicatedNode;
use bevy::prelude::{Query, Resource};
use common::input_packet::{Input, InputPacket};
use common::message_header::{DataType, MessageHeader};

#[derive(Resource)]
pub struct RpcManager {}

impl RpcManager {
    pub fn handle_rpc(
        &self,
        _addr: String,
        message_header: MessageHeader,
        buffer: Vec<u8>,
        mut replicated_nodes: Query<&mut ReplicatedNode>,
    ) {
        match message_header.get_data_type() {
            DataType::Rpc => {
                let input_packet = InputPacket::deserialize(buffer);
                if let Some(mut replicated_node) = replicated_nodes
                    .iter_mut()
                    .find(|node| node.net_id == input_packet.net_id)
                {
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
                }
            }
            DataType::Replication => {}
            DataType::None => {}
            DataType::Despawn => {}
        }
    }
}
