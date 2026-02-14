#[derive(Debug)]
pub enum MessageType {
    Helo,
    Hsk,
    Ping,
    Data,
}

pub struct MessageHeader {
    data: u8,
}

impl MessageHeader {
    pub fn new(message_type: MessageType, is_rpc: bool) -> Self {
        let mut data: u8 = message_type as u8;
        if is_rpc {
            data = data | 0x4;
        }
        println!("{}", data);
        MessageHeader { data }
    }

    pub fn from_data(data: u8) -> Self {
        Self { data }
    }

    pub fn get_message_type(&self) -> MessageType {
        let data = self.data;
        MessageType::try_from(data & 0x3).unwrap()
    }

    pub fn is_rpc(&self) -> bool {
        self.data & 0x4 == 0x4
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
            _ => Err(EnumError),
        }
    }
}
