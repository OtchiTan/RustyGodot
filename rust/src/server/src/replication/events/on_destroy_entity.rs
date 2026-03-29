use bevy::prelude::*;
use common::message_header::{DataType, MessageHeader, MessageType};
use common::stream_writer::StreamWriter;
use crate::network::connected_client::ConnectedClient;
use crate::network::network_manager::NetworkManager;
use crate::replication::replicated_nodes::player::Player;

#[derive(EntityEvent)]
pub struct DestroyEntity {
    pub(crate) entity: Entity,
}

pub fn on_destroy_entity(
    event: On<DestroyEntity>,
    connected_clients: Query<&ConnectedClient>,
    network_manager: Res<NetworkManager>,
    replicated_nodes: Query<&Player>,
    mut commands: Commands,
) {
    if let Ok(replicated_node) = replicated_nodes.get(event.entity) {
        let mut stream_writer = StreamWriter::new();
        let message_header = MessageHeader::init(MessageType::Data, DataType::Despawn);
        stream_writer.write_serializable(message_header);
        stream_writer.write_u32(replicated_node.net_id);

        for connected_client in connected_clients.iter() {
            network_manager.send_data(&connected_client.address, stream_writer.get_data());
        }

        commands.entity(event.entity).despawn();
    }
}
