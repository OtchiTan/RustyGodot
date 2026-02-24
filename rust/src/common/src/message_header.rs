#[derive(Debug, PartialEq, Eq)]
pub enum MessageType {
    Helo,
    Hsk,
    Ping,
    Data,
    Bye,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    None,
    Rpc,
    Replication,
    Despawn,
}

pub struct MessageHeader {
    data: u8,
}

impl MessageHeader {
    pub fn new(message_type: MessageType, data_type: DataType) -> Self {
        let mut data: u8 = (message_type as u8) << 2;
        data = data | (data_type as u8 & 0x3);
        MessageHeader { data }
    }

    pub fn from_data(data: u8) -> Self {
        Self { data }
    }

    pub fn get_message_type(&self) -> MessageType {
        let data = self.data;
        MessageType::try_from(data >> 2).unwrap()
    }

    pub fn get_data_type(&self) -> DataType {
        DataType::try_from(self.data & 0x3).unwrap()
    }

    pub fn get_data(&self) -> u8 {
        self.data
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