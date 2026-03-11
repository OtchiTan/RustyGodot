

pub struct StreamReader {
    buffer: Vec<u8>,
    cursor: usize,
}

impl StreamReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, cursor: 0 }
    }

    pub fn read_u8(&mut self) -> u8 {
        if self.cursor >= self.buffer.len() {
            return 0;
        }
        let data = self.buffer[self.cursor];
        self.cursor += 1;
        data
    }

    pub fn read_u16(&mut self) -> u16 {
        if self.cursor >= self.buffer.len() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 2];
        self.cursor += 2;
        u16::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_i16(&mut self) -> i16 {
        if self.cursor >= self.buffer.len() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 2];
        self.cursor += 2;
        i16::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_u32(&mut self) -> u32 {
        if self.cursor >= self.buffer.len() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 4];
        self.cursor += 4;
        u32::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_i32(&mut self) -> i32 {
        if self.cursor >= self.buffer.len() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 4];
        self.cursor += 4;
        i32::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_f32(&mut self) -> f32 {
        if self.cursor >= self.buffer.len() {
            return 0.0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 4];
        self.cursor += 4;
        f32::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_u64(&mut self) -> u64 {
        if self.cursor >= self.buffer.len() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 8];
        self.cursor += 8;
        u64::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_i64(&mut self) -> i64 {
        if self.cursor >= self.buffer.len() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 8];
        self.cursor += 8;
        i64::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_f64(&mut self) -> f64 {
        if self.cursor >= self.buffer.len() {
            return 0.0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 8];
        self.cursor += 8;
        f64::from_le_bytes(data.try_into().unwrap())
    }
}