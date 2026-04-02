use crate::network::connected_client::ConnectedClient;
use crate::network::network_manager::NetworkManager;
use crate::replication::replicated_nodes::player::Player;
use bevy::prelude::{Entity, Query, Res, Resource};
use common::message_header::{DataType, MessageHeader, MessageType};
use common::stream_writer::StreamWriter;
use std::collections::HashMap;

#[derive(Resource)]
pub struct ReplicationManager {
    pub client_entities: HashMap<u32, ClientEntityLink>,
}

pub struct ClientEntityLink {
    pub client: Entity,
    pub possessed_entity: HashMap<u32, Entity>,
    pub last_sequence: u32,
}

impl ClientEntityLink {
    pub fn new(client: Entity) -> Self {
        Self {
            client,
            possessed_entity: HashMap::new(),
            last_sequence: 0,
        }
    }
}

pub fn update_replication(
    network_manager: Res<NetworkManager>,
    clients: Query<&ConnectedClient>,
    replicated_nodes: Query<&Player>,
) {
    let message_header = MessageHeader::init(MessageType::Data, DataType::Replication);
    let mut stream_writer = StreamWriter::new();
    stream_writer.write_serializable(message_header);

    for player in replicated_nodes.iter() {
        stream_writer.write_serializable_ref(player);
    }

    for client in clients.iter() {
        network_manager.send_data(&client.address, stream_writer.get_data());
    }
}
