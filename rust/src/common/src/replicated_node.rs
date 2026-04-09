use crate::stream_reader::{Deserializable, StreamReader};
use crate::stream_writer::{Serializable, StreamWriter};

#[derive(Debug, Clone)]
pub struct ReplicatedNode {
    pub net_id: u32,
    pub type_id: u32,
    pub data: Vec<u8>,
}

impl Serializable for ReplicatedNode {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u32(self.net_id);
        stream.write_u32(self.type_id);
        stream.write_serializable_vec(self.data.clone());
    }
}

impl Deserializable for ReplicatedNode {
    fn deserialize(stream_reader: &mut StreamReader) -> Self {
        let net_id = stream_reader.read_u32();
        let type_id = stream_reader.read_u32();
        let data: Vec<u8> = stream_reader.read_serializable_vec();

        Self {
            net_id,
            type_id,
            data,
        }
    }
}
