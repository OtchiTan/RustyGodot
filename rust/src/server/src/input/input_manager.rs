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

        println!("Buffer Len : {}", buffers.len());

        for buffer in buffers {
            println!("{:?}", buffer);
            if let Some((mut player, mut velocity)) = players
                .iter_mut()
                .find(|(player, _)| player.net_id == buffer.node_id)
            {
                velocity.linvel = Vec2::new(0.0, 0.0);
                for input_packet in buffer.packets {
                    if input_packet.sequence <= self.server_frame
                        && input_packet.sequence >= player.last_handled_input
                    {
                        player.last_handled_input = input_packet.sequence;
                        velocity.linvel += player.handle_input(input_packet);
                    }
                }
                println!("{}", velocity.linvel);
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
