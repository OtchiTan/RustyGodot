use crate::input::input_manager::InputManager;
use crate::network::connected_client::ConnectedClient;
use crate::network::network_manager::NetworkManager;
use crate::replication::replicated_nodes::player::Player;
use bevy::prelude::{Entity, Query, Res, Resource, Transform};
use bevy_rapier2d::prelude::Velocity;
use common::message_header::{DataType, MessageHeader, MessageType};
use common::replicated_node::ReplicatedNode;
use common::snapshot::Snapshot;
use common::stream_writer::StreamWriter;
use glm::Vec2;
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
    replicated_nodes: Query<(&Transform, &Player, &Velocity)>,
    input_manager: Res<InputManager>,
) {
    let mut stream_writer = StreamWriter::new();

    let message_header = MessageHeader::init(MessageType::Data, DataType::Replication);
    stream_writer.write_serializable(message_header);

    let mut snapshot = Snapshot::new(input_manager.server_frame);

    for (transform, player, velocity) in replicated_nodes.iter() {
        let mut sw = StreamWriter::new();
        sw.write_vec2(Vec2::new(transform.translation.x, transform.translation.y));
        sw.write_vec2(Vec2::new(velocity.linvel.x, velocity.linvel.y));
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
