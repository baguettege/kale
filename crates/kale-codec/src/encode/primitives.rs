use crate::encode::{Encode, Encoder};

macro_rules! impl_encode {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Encode for $ty {
                fn encode(&self, encoder: &mut Encoder) {
                    encoder.put(self.to_be_bytes());
                }
            }
        )*
    };
}

impl_encode!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Encode for usize {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&(*self as u64));
    }
}

impl Encode for bool {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&(*self as u8));
    }
}

impl Encode for char {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&(*self as u32));
    }
}
