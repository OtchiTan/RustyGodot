pub trait Serializable {
    fn serialize(&self, stream: &mut StreamWriter);
}

pub struct StreamWriter {
    buffer: Vec<u8>,
}

impl StreamWriter {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.buffer
    }

    pub fn write_u8(&mut self, data: u8) {
        self.buffer.push(data);
    }

    pub fn write_u16(&mut self, data: u16) {
        self.buffer
            .extend_from_slice(u16::to_le_bytes(data).as_ref());
    }

    pub fn write_i16(&mut self, data: i16) {
        self.buffer
            .extend_from_slice(i16::to_le_bytes(data).as_ref());
    }

    pub fn write_u32(&mut self, data: u32) {
        self.buffer
            .extend_from_slice(u32::to_le_bytes(data).as_ref());
    }

    pub fn write_i32(&mut self, data: i32) {
        self.buffer
            .extend_from_slice(i32::to_le_bytes(data).as_ref());
    }

    pub fn write_f32(&mut self, data: f32) {
        self.buffer
            .extend_from_slice(f32::to_le_bytes(data).as_ref());
    }

    pub fn write_u64(&mut self, data: u64) {
        self.buffer
            .extend_from_slice(u64::to_le_bytes(data).as_ref());
    }

    pub fn write_i64(&mut self, data: i64) {
        self.buffer
            .extend_from_slice(i64::to_le_bytes(data).as_ref());
    }

    pub fn write_f64(&mut self, data: f64) {
        self.buffer
            .extend_from_slice(f64::to_le_bytes(data).as_ref());
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn write_serializable<T: Serializable>(&mut self, data: T) {
        data.serialize(self);
    }
}
