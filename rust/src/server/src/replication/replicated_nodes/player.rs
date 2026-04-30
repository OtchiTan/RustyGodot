use bevy::prelude::Component;
use common::input_packet::InputPacket;
use common::stream_writer::{Serializable, StreamWriter};
use std::collections::VecDeque;

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub net_id: u32,
    pub type_id: u32,
    pub owner_id: u32,
    pub input_queue: VecDeque<InputPacket>,
    pub last_queued_input: u32,
    pub player_speed: f32,
}

impl Player {
    pub fn new(net_id: u32, owner_id: u32) -> Self {
        Self {
            net_id,
            type_id: 0,
            owner_id,
            last_queued_input: 0,
            input_queue: VecDeque::new(),
            player_speed: 500.0,
        }
    }
}

impl Serializable for Player {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u32(self.owner_id);
    }
}
