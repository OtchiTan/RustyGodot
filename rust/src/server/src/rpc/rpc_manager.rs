use crate::replication::replication_manager::ReplicationManager;
use bevy::prelude::Resource;
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
        replication_manager: &ReplicationManager,
    ) {
        match message_header.get_data_type() {
            DataType::Rpc => {
                let mut serializer = Serializer::new(buffer.to_vec());
                let net_id: u32 = 0;
                let class_id: usize = 0;
                let x: f32 = 0.0;
                let y: f32 = 0.0;
                let _ = &mut serializer << net_id;
                let _ = &mut serializer << class_id;
                let _ = &mut serializer << x;
                let _ = &mut serializer << y;

                if let Some(_entity) = replication_manager.map.get(&net_id) {}
            }
            DataType::Replication => {}
            DataType::None => {}
        }
    }
}
