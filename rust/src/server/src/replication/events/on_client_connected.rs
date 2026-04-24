use crate::replication::replicated_nodes::player::Player;
use crate::replication::replication_manager::{ClientEntityLink, ReplicationManager};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::prelude::{Collider, RigidBody};

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
        let position = Transform::from_xyz(
            rand::random_range(20.0..180.0) * 16.0,
            rand::random_range(20.0..90.0) * 16.0,
            0.0,
        );
        let player = Player::new(player_net_id, on_connected.client_net_id);

        let player_entity = commands
            .spawn((
                player,
                RigidBody::Dynamic,
                Collider::ball(15.0),
                position,
                Velocity {
                    linvel: Vec2::new(0.0, 0.0),
                    angvel: 0.0,
                },
            ))
            .id();

        let mut client_entity = ClientEntityLink::new(on_connected.entity);

        client_entity
            .possessed_entity
            .insert(player_net_id, player_entity);

        replication_manager
            .client_entities
            .insert(on_connected.client_net_id, client_entity);
    }
}
