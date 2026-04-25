use crate::tag::{AstTag, LiteralTag};

macro_rules! impl_encode {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::encode::Encode for $ty {
                fn encode(&self, encoder: &mut $crate::encode::Encoder) {
                    encoder.encode(&(*self as u8));
                }
            }
        )*
    };
}

impl_encode!(AstTag, LiteralTag);
