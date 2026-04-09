use glm::Vec2;

pub trait Deserializable {
    fn deserialize(stream_reader: &mut StreamReader) -> Self;
}

pub struct StreamReader {
    buffer: Vec<u8>,
    cursor: usize,
}

impl StreamReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, cursor: 0 }
    }

    pub fn read_u8(&mut self) -> u8 {
        if !self.remain_data() {
            return 0;
        }
        let data = self.buffer[self.cursor];
        self.cursor += 1;
        data
    }

    pub fn read_u16(&mut self) -> u16 {
        if !self.remain_data() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 2];
        self.cursor += 2;
        u16::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_i16(&mut self) -> i16 {
        if !self.remain_data() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 2];
        self.cursor += 2;
        i16::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_u32(&mut self) -> u32 {
        if !self.remain_data() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 4];
        self.cursor += 4;
        u32::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_i32(&mut self) -> i32 {
        if !self.remain_data() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 4];
        self.cursor += 4;
        i32::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_f32(&mut self) -> f32 {
        if !self.remain_data() {
            return 0.0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 4];
        self.cursor += 4;
        f32::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_u64(&mut self) -> u64 {
        if !self.remain_data() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 8];
        self.cursor += 8;
        u64::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_i64(&mut self) -> i64 {
        if !self.remain_data() {
            return 0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 8];
        self.cursor += 8;
        i64::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_f64(&mut self) -> f64 {
        if !self.remain_data() {
            return 0.0;
        }
        let data = &self.buffer[self.cursor..self.cursor + 8];
        self.cursor += 8;
        f64::from_le_bytes(data.try_into().unwrap())
    }

    pub fn read_vec2(&mut self) -> Vec2 {
        if !self.remain_data() {
            return Vec2::new(0.0, 0.0);
        }
        let x = self.read_f32();
        let y = self.read_f32();
        Vec2::new(x, y)
    }

    pub fn read_serializable<T: Deserializable>(&mut self) -> T {
        T::deserialize(self)
    }

    pub fn read_serializable_vec<T: Deserializable>(&mut self) -> Vec<T> {
        let len = self.read_u32() as usize;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(self.read_serializable());
        }

        vec
    }

    pub fn get_rest_buffer(&self) -> &[u8] {
        &self.buffer[self.cursor..]
    }

    pub fn remain_data(&self) -> bool {
        self.cursor < self.buffer.len()
    }
}

impl Deserializable for u8 {
    fn deserialize(stream_reader: &mut StreamReader) -> Self {
        stream_reader.read_u8()
    }
}
