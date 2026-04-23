use bevy::prelude::Component;
use common::input_packet::{Input, InputPacket};
use common::stream_writer::{Serializable, StreamWriter};
use glm::Vec2;

#[derive(Component)]
pub struct Player {
    pub net_id: u32,
    pub type_id: u32,
    pub owner_id: u32,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            net_id: self.net_id,
            type_id: self.type_id,
            owner_id: self.owner_id,
            position: self.position,
            velocity: self.velocity,
        }
    }
}

impl Player {
    pub fn new(net_id: u32, owner_id: u32, position: Vec2) -> Self {
        Self {
            net_id,
            type_id: 0,
            owner_id,
            position,
            velocity: Vec2::new(0.0, 0.0),
        }
    }

    pub fn handle_input(&mut self, input_packet: InputPacket) {
        let input_direction =
            input_packet.read_vector(Input::Right, Input::Left, Input::Up, Input::Down);
        self.velocity = input_direction * 500.0;
    }
}

impl Serializable for Player {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_vec2(self.position);
        stream.write_u32(self.owner_id);
    }
}
