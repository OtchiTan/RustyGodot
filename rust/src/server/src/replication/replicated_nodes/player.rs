use bevy::prelude::Component;
use common::input_packet::{Input, InputPacket};
use common::stream_writer::{Serializable, StreamWriter};
use glm::Vec2;

#[derive(Component, Clone)]
pub struct Player {
    pub net_id: u32,
    pub type_id: u32,
    pub owner_id: u32,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Player {
    pub fn handle_input(&mut self, input_packet: InputPacket) {
        let input_direction =
            input_packet.read_vector(Input::Right, Input::Left, Input::Down, Input::Up);
        self.velocity = input_direction * 5.0;
        self.position = self.position + self.velocity;
    }
}

impl Serializable for Player {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_vec2(self.position);
        stream.write_vec2(self.velocity);
        stream.write_u32(self.owner_id);
    }
}
