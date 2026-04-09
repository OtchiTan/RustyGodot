use crate::replication::replicated_nodes::player::Player;
use crate::replication::replication_manager::{ClientEntityLink, ReplicationManager};
use bevy::prelude::*;
use glm::Vec2;

#[derive(Message, Debug)]
pub struct ClientConnected {
    pub entity: Entity,
    pub client_net_id: u32,
}

pub fn on_client_connected(
    mut messages: MessageReader<ClientConnected>,
    mut commands: Commands,
    mut replication_manager: ResMut<ReplicationManager>,
) {
    for on_connected in messages.read() {
        let player_net_id = rand::random();
        let position = Vec2::new(
            rand::random_range(20.0..180.0) * 16.0,
            rand::random_range(20.0..90.0) * 16.0,
        );
        let player = Player {
            net_id: player_net_id,
            type_id: 0,
            owner_id: on_connected.client_net_id,
            position,
        };

        let player_entity = commands.spawn(player).id();

        let mut client_entity = ClientEntityLink::new(on_connected.entity);

        client_entity
            .possessed_entity
            .insert(player_net_id, player_entity);

        replication_manager
            .client_entities
            .insert(on_connected.client_net_id, client_entity);
    }
}
