use crate::tag::{AstTag, LiteralTag};

macro_rules! impl_decode {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::decode::Decode for $ty {
                fn decode(decoder: &mut $crate::decode::Decoder) -> $crate::Result<Self> {
                    let tag = decoder.decode::<u8>()?;
                    Self::try_from(tag).map_err(|_| $crate::Error::UnknownTag(tag))
                }
            }
        )*
    };
}

impl_decode!(AstTag, LiteralTag);
