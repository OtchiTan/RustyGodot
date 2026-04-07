use crate::stream_reader::{Deserializable, StreamReader};
use crate::stream_writer::{Serializable, StreamWriter};
use glm::Vec2;

#[derive(Debug)]
pub struct InputBuffer {
    pub node_id: u32,
   pub packets: Vec<InputPacket>,
}

#[derive(Debug, Clone)]
pub struct InputPacket {
    pub sequence: u32,
    pub keys: u8, // bitfield : bit0=haut, bit1=bas, bit2=gauche, bit3=droite
    pub aim_x: f32,
    pub aim_y: f32,
}

#[derive(Debug)]
pub enum Input {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl InputPacket {
    pub fn new() -> Self {
        Self {
            sequence: 0,
            keys: 0,
            aim_x: 0.0,
            aim_y: 0.0,
        }
    }

    pub fn add_input(&mut self, input: Input) {
        self.keys = self.keys | 1u8 << (input as u8);
    }

    pub fn read_input(&self, input: Input) -> bool {
        self.keys & 1u8 << (input as u8) != 0
    }

    pub fn read_axis(&self, negative: Input, positive: Input) -> f32 {
        if self.read_input(negative) {
            -1.0
        } else if self.read_input(positive) {
            1.0
        } else {
            0.0
        }
    }

    pub fn read_vector(
        &self,
        x_positive: Input,
        x_negative: Input,
        y_positive: Input,
        y_negative: Input,
    ) -> Vec2 {
        let x = self.read_axis(x_negative, x_positive);
        let y = self.read_axis(y_negative, y_positive);
        Vec2::new(x, y)
    }

    pub fn reset(&mut self) {
        self.sequence = 0;
        self.keys = 0;
        self.aim_x = 0.0;
        self.aim_y = 0.0;
    }
}

impl Serializable for InputBuffer {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u32(self.node_id);
        stream.write_serializable_vec(self.packets.clone());
    }
}

impl Deserializable for InputBuffer {
    fn deserialize(stream_reader: &mut StreamReader) -> Self {
        let node_id = stream_reader.read_u32();
        let packets = stream_reader.read_serializable_vec();

        Self { node_id, packets }
    }
}

impl Serializable for InputPacket {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u32(self.sequence);
        stream.write_u8(self.keys);
        stream.write_f32(self.aim_x);
        stream.write_f32(self.aim_y);
    }
}

impl Deserializable for InputPacket {
    fn deserialize(stream_reader: &mut StreamReader) -> Self {
        let sequence = stream_reader.read_u32();
        let keys = stream_reader.read_u8();
        let aim_x = stream_reader.read_f32();
        let aim_y = stream_reader.read_f32();

        Self {
            sequence,
            keys,
            aim_x,
            aim_y,
        }
    }
}
