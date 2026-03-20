use crate::stream_reader::StreamReader;
use crate::stream_writer::StreamWriter;

#[derive(Debug, Clone)]
pub struct InputPacket {
    pub net_id: u32,
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
    pub fn new(net_id: u32) -> Self {
        Self {
            net_id,
            sequence: 0,
            keys: 0,
            aim_x: 0.0,
            aim_y: 0.0,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut stream_writer = StreamWriter::new(vec![]);

        stream_writer.write_u32(self.net_id);
        stream_writer.write_u32(self.sequence);
        stream_writer.write_u8(self.keys);
        stream_writer.write_f32(self.aim_x);
        stream_writer.write_f32(self.aim_y);

        stream_writer.get_data().to_vec()
    }

    pub fn deserialize(data: Vec<u8>) -> InputPacket {
        let mut stream_reader = StreamReader::new(data);

        let net_id = stream_reader.read_u32();
        let sequence = stream_reader.read_u32();
        let keys = stream_reader.read_u8();
        let aim_x = stream_reader.read_f32();
        let aim_y = stream_reader.read_f32();

        Self {
            net_id,
            sequence,
            keys,
            aim_x,
            aim_y,
        }
    }

    pub fn add_input(&mut self, input: Input) {
        self.keys = self.keys | 1u8 << (input as u8);
    }

    pub fn read_input(&self, input: Input) -> bool {
        self.keys & 1u8 << (input as u8) != 0
    }
}
