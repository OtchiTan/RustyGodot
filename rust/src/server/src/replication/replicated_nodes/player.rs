use bevy::prelude::{Component};
use common::input_packet::{Input, InputPacket};
use common::stream_writer::{Serializable, StreamWriter};
use glm::Vec2;

#[derive(Component)]
pub struct Player {
    pub net_id: u32,
    pub type_id: u32,
    pub owner_id: u32,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            net_id: self.net_id,
            type_id: self.type_id,
            owner_id: self.owner_id,
        }
    }
}

impl Player {
    pub fn new(net_id: u32, owner_id: u32) -> Self {
        Self {
            net_id,
            type_id: 0,
            owner_id,
        }
    }

    pub fn handle_input(&mut self, input_packet: InputPacket) -> Vec2 {
        let input_direction =
            input_packet.read_vector(Input::Right, Input::Left, Input::Up, Input::Down);
        input_direction * 500.0
    }
}

impl Serializable for Player {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u32(self.owner_id);
    }
}