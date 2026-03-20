use crate::stream_reader::{Deserializable, StreamReader};
use crate::stream_writer::{Serializable, StreamWriter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    Helo = 0,
    Hsk = 1,
    Ping = 2,
    Data = 3,
    Bye = 4,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataType {
    None = 0,
    Rpc = 1,
    Replication = 2,
    Despawn = 3,
}

pub struct MessageHeader {
    pub message_type: MessageType,
    pub data_type: DataType,
}

impl MessageHeader {
    pub fn new() -> Self {
        Self {
            message_type: MessageType::Helo,
            data_type: DataType::None,
        }
    }
    pub fn init(message_type: MessageType, data_type: DataType) -> Self {
        Self {
            message_type,
            data_type,
        }
    }
}

#[derive(Debug)]
pub struct EnumError;

impl TryFrom<u8> for MessageType {
    type Error = EnumError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::Helo),
            1 => Ok(MessageType::Hsk),
            2 => Ok(MessageType::Ping),
            3 => Ok(MessageType::Data),
            4 => Ok(MessageType::Bye),
            _ => Err(EnumError),
        }
    }
}

impl TryFrom<u8> for DataType {
    type Error = EnumError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DataType::None),
            1 => Ok(DataType::Rpc),
            2 => Ok(DataType::Replication),
            3 => Ok(DataType::Despawn),
            _ => Err(EnumError),
        }
    }
}

impl Serializable for MessageHeader {
    fn serialize(&self, stream: &mut StreamWriter) {
        stream.write_u8(self.message_type as u8);
        stream.write_u8(self.data_type as u8);
    }
}

impl Deserializable for MessageHeader {
    fn deserialize(&mut self, stream: &mut StreamReader) {
        self.message_type = MessageType::try_from(stream.read_u8()).unwrap();
        self.data_type = DataType::try_from(stream.read_u8()).unwrap();
    }
}