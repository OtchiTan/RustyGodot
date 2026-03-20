use crate::network::connected_client::ConnectedClient;
use crate::network::network_manager::NetworkManager;
use crate::replication::replicated_node::ReplicatedNode;
use crate::replication::replication_manager::ReplicationManager;
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{Commands, Entity, EntityEvent, Fixed, On, Query, Res, Time};
use common::message_header::{DataType, MessageHeader, MessageType};
use common::stream_writer::StreamWriter;
use std::collections::HashMap;

pub mod replicated_node;
pub mod replication_manager;

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReplicationManager {
            client_entities: HashMap::new(),
        })
        .add_observer(on_destroy_entity)
        .insert_resource(Time::<Fixed>::from_hz(30.0))
        .add_systems(FixedUpdate, update_replication);
    }
}

fn update_replication(
    network_manager: Res<NetworkManager>,
    clients: Query<&ConnectedClient>,
    replicated_nodes: Query<&ReplicatedNode>,
) {
    for replicated_node in replicated_nodes.iter() {
        let message_header = MessageHeader::init(MessageType::Data, DataType::Replication);
        let mut stream_writer = StreamWriter::new(vec![]);
        stream_writer.write_serializable(message_header);
        stream_writer.write_u32(replicated_node.net_id);
        stream_writer.write_u32(replicated_node.type_id);
        stream_writer.write_f32(replicated_node.x);
        stream_writer.write_f32(replicated_node.y);
        stream_writer.write_u32(replicated_node.owner_id);
        for client in clients.iter() {
            network_manager.send_data(&client.address, stream_writer.get_data());
        }
    }
}

#[derive(EntityEvent)]
pub struct DestroyEntity {
    pub(crate) entity: Entity,
}

fn on_destroy_entity(
    event: On<DestroyEntity>,
    connected_clients: Query<&ConnectedClient>,
    network_manager: Res<NetworkManager>,
    replicated_nodes: Query<&ReplicatedNode>,
    mut commands: Commands,
) {
    if let Ok(replicated_node) = replicated_nodes.get(event.entity) {
        let mut stream_writer = StreamWriter::new(vec![]);
        let message_header = MessageHeader::init(MessageType::Data, DataType::Despawn);
        stream_writer.write_serializable(message_header);
        stream_writer.write_u32(replicated_node.net_id);

        for connected_client in connected_clients.iter() {
            network_manager.send_data(&connected_client.address, stream_writer.get_data());
        }

        commands.entity(event.entity).despawn();
    }
}
