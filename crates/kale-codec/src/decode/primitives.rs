use crate::decode::{Decode, Decoder};
use crate::{Error, Result};

macro_rules! impl_decode {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Decode for $ty {
                fn decode(decoder: &mut Decoder) -> Result<Self> {
                    const SIZE: usize = size_of::<$ty>();
                    let bytes = decoder.take_array::<SIZE>()?;
                    Ok(Self::from_be_bytes(*bytes))
                }
            }
        )*
    };
}

impl_decode!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl Decode for usize {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        Ok(decoder.decode::<u64>()? as usize)
    }
}

impl Decode for bool {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        match decoder.decode::<u8>()? {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(Error::InvalidData(format!("invalid bool: {v}")))
        }
    }
}
