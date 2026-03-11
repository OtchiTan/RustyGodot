use crate::replication::replicated_node::ReplicatedNode;
use bevy::prelude::{Query, Resource};
use common::message_header::{DataType, MessageHeader};
use common::stream_reader::StreamReader;

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
                let mut stream_reader = StreamReader::new(buffer);
                let net_id = stream_reader.read_u32();
                let x = stream_reader.read_f32();
                let y = stream_reader.read_f32();

                if let Some(mut replicated_node) = replicated_nodes
                    .iter_mut()
                    .find(|node| node.net_id == net_id)
                {
                    replicated_node.x = x;
                    replicated_node.y = y;
                }
            }
            DataType::Replication => {}
            DataType::None => {}
            DataType::Despawn => {}
        }
    }
}
