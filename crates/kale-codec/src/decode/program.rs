use kale_syntax::ast::{Block, Program};
use crate::decode::{Decode, Decoder};
use crate::Result;

impl Decode for Program {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        Ok(Self(decoder.decode::<Block>()?))
    }
}
