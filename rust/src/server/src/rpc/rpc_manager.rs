use crate::replication::replicated_node::ReplicatedNode;
use bevy::prelude::{Query, Resource};
use common::message_header::{DataType, MessageHeader};
use common::serializer::Serializer;

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
                let mut serializer = Serializer::new(buffer.to_vec());
                let mut net_id: u32 = 0;
                let mut x: f32 = 0.0;
                let mut y: f32 = 0.0;
                let _ = &mut serializer >> &mut net_id;
                let _ = &mut serializer >> &mut x;
                let _ = &mut serializer >> &mut y;

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
