use crate::encode::{Encode, Encoder};
use crate::tag::LiteralTag;
use kale_syntax::ast::{BinOp, Ident, Literal, UnOp};

impl Encode for Ident {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&self.as_bytes().to_vec());
    }
}

impl Encode for BinOp {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&(*self as u8));
    }
}

impl Encode for UnOp {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&(*self as u8));
    }
}

impl Encode for Literal {
    fn encode(&self, encoder: &mut Encoder) {
        fn tagged<T: Encode>(encoder: &mut Encoder, tag: LiteralTag, value: &T) {
            encoder
                .encode(&tag)
                .encode(value);
        }

        match self {
            Self::Nil => { encoder.encode(&LiteralTag::Nil); },
            Self::Num(n) => tagged(encoder, LiteralTag::Num, n),
            Self::Bool(b) => tagged(encoder, LiteralTag::Bool, b),
            Self::Char(c) => tagged(encoder, LiteralTag::Char, c),
            Self::Str(s) => tagged(encoder, LiteralTag::Str, s),
        }
    }
}
