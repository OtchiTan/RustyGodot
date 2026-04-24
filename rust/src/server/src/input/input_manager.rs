use crate::network::connected_client::ConnectedClient;
use crate::replication::replicated_nodes::player::Player;
use bevy::prelude::{Query, Resource, Vec2};
use bevy_rapier2d::prelude::Velocity;
use common::input_packet::InputBuffer;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Resource)]
pub struct InputManager {
    pub server_frame: u32,
}

impl InputManager {
    pub fn handle_input(
        &mut self,
        buffers: Vec<InputBuffer>,
        mut players: Query<(&mut Player, &mut Velocity)>,
        mut clients: Query<&mut ConnectedClient>,
    ) {
        let server_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        for buffer in buffers {
            if let Some(mut player) = players
                .iter_mut()
                .find(|node| node.0.net_id == buffer.node_id)
            {
                for input_packet in buffer.packets {
                    if input_packet.sequence == self.server_frame {
                        let velocity = player.0.handle_input(input_packet);
                        player.1.linvel = Vec2::new(velocity.x, velocity.y);
                    }
                }
            }

            if let Some(mut client) = clients
                .iter_mut()
                .find(|client| client.net_id == buffer.client_id)
            {
                client.latest_data_received = server_time;
            }
        }

        self.server_frame += 1;
    }
}
