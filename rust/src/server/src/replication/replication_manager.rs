use crate::network::connected_client::ConnectedClient;
use crate::network::network_manager::NetworkManager;
use crate::replication::replicated_nodes::player::Player;
use crate::rpc::input_manager::InputManager;
use bevy::prelude::{Entity, Query, Res, Resource};
use common::message_header::{DataType, MessageHeader, MessageType};
use common::replicated_node::ReplicatedNode;
use common::snapshot::Snapshot;
use common::stream_writer::StreamWriter;
use std::collections::HashMap;

#[derive(Resource)]
pub struct ReplicationManager {
    pub client_entities: HashMap<u32, ClientEntityLink>,
}

pub struct ClientEntityLink {
    pub client: Entity,
    pub possessed_entity: HashMap<u32, Entity>,
}

impl ClientEntityLink {
    pub fn new(client: Entity) -> Self {
        Self {
            client,
            possessed_entity: HashMap::new(),
        }
    }
}

pub fn handle_snapshots(
    network_manager: Res<NetworkManager>,
    clients: Query<&ConnectedClient>,
    replicated_nodes: Query<&Player>,
    input_manager: Res<InputManager>,
) {
    let mut stream_writer = StreamWriter::new();

    let message_header = MessageHeader::init(MessageType::Data, DataType::Replication);
    stream_writer.write_serializable(message_header);

    let mut snapshot = Snapshot::new(input_manager.server_frame);

    for player in replicated_nodes.iter() {
        let mut sw = StreamWriter::new();
        sw.write_serializable_ref(player);

        snapshot.nodes.push(ReplicatedNode {
            net_id: player.net_id,
            type_id: player.type_id,
            data: sw.get_data().to_vec(),
        });
    }

    stream_writer.write_serializable(snapshot);

    for client in clients.iter() {
        network_manager.send_data(&client.address, stream_writer.get_data());
    }
}
