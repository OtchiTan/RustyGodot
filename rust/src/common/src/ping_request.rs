use crate::stream_reader::{Deserializable, StreamReader};
use crate::stream_writer::{Serializable, StreamWriter};

#[derive(Debug)]
pub struct PingRequest {
    pub time_client_request: u64,
}

impl Serializable for PingRequest {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u64(self.time_client_request);
    }
}

impl Deserializable for PingRequest {
    fn deserialize(stream: &mut StreamReader) -> Self {
        let time_client_request = stream.read_u64();
        Self {
            time_client_request,
        }
    }
}

#[derive(Debug)]
pub struct PingResponse {
    pub time_client_request: u64,
    pub time_server_response: u64,
    pub server_frame: u32,
}

impl Serializable for PingResponse {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u64(self.time_client_request);
        stream.write_u64(self.time_server_response);
        stream.write_u32(self.server_frame);
    }
}

impl Deserializable for PingResponse {
    fn deserialize(stream: &mut StreamReader) -> Self {
        let time_client_request = stream.read_u64();
        let time_server_response = stream.read_u64();
        let server_frame = stream.read_u32();

        Self {
            time_client_request,
            time_server_response,
            server_frame,
        }
    }
}
