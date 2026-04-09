use crate::stream_reader::{Deserializable, StreamReader};
use crate::stream_writer::{Serializable, StreamWriter};

#[derive(Debug)]
pub struct Handshake {
    pub client_id: u32,
    pub server_frequency: f64,
}

impl Serializable for Handshake {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u32(self.client_id);
        stream.write_f64(self.server_frequency);
    }
}

impl Deserializable for Handshake {
    fn deserialize(stream_reader: &mut StreamReader) -> Self {
        Self {
            client_id: stream_reader.read_u32(),
            server_frequency: stream_reader.read_f64(),
        }
    }
}
