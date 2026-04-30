use crate::network::connected_client::ConnectedClient;
use crate::replication::replicated_nodes::player::Player;
use bevy::prelude::{Query, Resource, Vec2};
use bevy_rapier2d::prelude::Velocity;
use common::input_packet::{Input, InputBuffer};
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
            if let Some((mut player, _)) = players
                .iter_mut()
                .find(|(player, _)| player.net_id == buffer.node_id)
            {
                for input_packet in buffer.packets {
                    if input_packet.sequence <= self.server_frame
                        && input_packet.sequence > player.last_queued_input
                    {
                        player.last_queued_input = input_packet.sequence;
                        player.input_queue.push_back(input_packet);
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

        for (mut player, mut velocity) in players.iter_mut() {
            let input_count = player.input_queue.len();
            let mut total_direction = Vec2::new(0.0, 0.0);
            while let Some(next_input) = player.input_queue.pop_front() {
                let input_direction =
                    next_input.read_vector(Input::Right, Input::Left, Input::Up, Input::Down);
                total_direction += Vec2::new(input_direction.x, input_direction.y);
            }
            if input_count > 0 {
                let average_direction = (total_direction / input_count as f32).normalize_or_zero();
                let target_velocity = average_direction * player.player_speed;

                velocity.linvel = velocity.linvel.lerp(target_velocity, 0.3);
            }
        }

        self.server_frame += 1;
    }
}
