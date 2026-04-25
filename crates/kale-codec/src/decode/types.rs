use kale_syntax::ast::{BinOp, Ident, Literal, UnOp};
use crate::decode::{Decode, Decoder};
use crate::{Error, Result};
use crate::tag::LiteralTag;

impl Decode for Ident {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let bytes = decoder.decode::<Vec<u8>>()?;
        Ok(std::str::from_utf8(&bytes)?.to_string())
    }
}

impl Decode for BinOp {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let tag = decoder.decode::<u8>()?;
        Self::try_from(tag).map_err(|_| Error::InvalidData(format!("invalid binop tag: {tag}")))
    }
}

impl Decode for UnOp {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let tag = decoder.decode::<u8>()?;
        Self::try_from(tag).map_err(|_| Error::InvalidData(format!("invalid unop tag: {tag}")))
    }
}

impl Decode for Literal {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        match decoder.decode::<LiteralTag>()? {
            LiteralTag::Nil => Ok(Self::Nil),
            LiteralTag::Num => Ok(Self::Num(decoder.decode()?)),
            LiteralTag::Bool => Ok(Self::Bool(decoder.decode()?)),
            LiteralTag::Str => Ok(Self::Str(decoder.decode()?)),
        }
    }
}
