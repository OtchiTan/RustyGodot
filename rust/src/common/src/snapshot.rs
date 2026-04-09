use crate::replicated_node::ReplicatedNode;
use crate::stream_reader::{Deserializable, StreamReader};
use crate::stream_writer::{Serializable, StreamWriter};

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub frame: u32,
    pub nodes: Vec<ReplicatedNode>,
}

impl Snapshot {
    pub fn new(frame: u32) -> Snapshot {
        Snapshot {
            frame,
            nodes: Vec::new(),
        }
    }
}

impl Serializable for Snapshot {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u32(self.frame);
        stream.write_serializable_vec(self.nodes.clone());
    }
}

impl Deserializable for Snapshot {
    fn deserialize(stream_reader: &mut StreamReader) -> Self {
        let frame = stream_reader.read_u32();
        let players = stream_reader.read_serializable_vec();

        Snapshot {
            frame,
            nodes: players,
        }
    }
}
