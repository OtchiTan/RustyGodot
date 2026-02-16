use num_traits::ToBytes;
use std::ops::{Shl, Shr};

pub struct Serializer {
    buffer: Vec<u8>,
    cursor: usize,
}

impl Serializer {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, cursor: 0 }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.buffer
    }
}

impl<'a, T: ToBytes> Shl<T> for &'a mut Serializer {
    type Output = &'a mut Serializer;

    fn shl(self, rhs: T) -> Self::Output {
        self.buffer.extend_from_slice(rhs.to_le_bytes().as_ref());
        self
    }
}

macro_rules! impl_shr {
    ($($t:ty),*) => {
        $(
            impl<'a> Shr<&'a mut $t> for &'a mut Serializer {
                type Output = &'a mut Serializer;

                fn shr(self, rhs: &'a mut $t) -> Self::Output {
                    let size = std::mem::size_of::<$t>();
                    let bytes = &self.buffer[self.cursor..self.cursor + size];

                    *rhs = <$t>::from_le_bytes(bytes.try_into().unwrap());

                    self.cursor += size;
                    self
                }
            }
        )*
    };
}

impl_shr!(u8, u16, u32, u64, i32, f32, f64);
