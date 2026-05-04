use kale_syntax::span::{Span, Spanned};
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
            b => Err(Error::InvalidData(format!("invalid bool: {b}")))
        }
    }
}

impl Decode for char {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let b = decoder.decode::<u32>()?;
        char::from_u32(b).ok_or_else(|| Error::InvalidData(
            format!("invalid char as u32: {b}")))
    }
}

impl Decode for Span {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let start = decoder.decode()?;
        let end = decoder.decode()?;
        Ok(Self::new(start, end))
    }
}

impl<T: Decode> Decode for Spanned<T> {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let span = decoder.decode()?;
        let inner = decoder.decode()?;
        Ok(Self::new(span, inner))
    }
}
