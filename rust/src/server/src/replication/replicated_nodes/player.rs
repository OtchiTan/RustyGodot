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
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            net_id: self.net_id,
            type_id: self.type_id,
            owner_id: self.owner_id,
            position: self.position,
        }
    }
}

impl Player {
    pub fn handle_input(&mut self, input_packet: InputPacket) {
        let input_direction =
            input_packet.read_vector(Input::Right, Input::Left, Input::Down, Input::Up);
        self.position = self.position + input_direction * 5.0;
    }
}

impl Serializable for Player {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u32(self.net_id);
        stream.write_u32(self.type_id);
        stream.write_vec2(self.position);
        stream.write_u32(self.owner_id);
    }
}
