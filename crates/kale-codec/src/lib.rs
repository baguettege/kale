mod decode;
mod encode;
mod error;
mod tag;

pub use error::{Error, Result};
use crate::decode::Decoder;
use crate::encode::Encoder;
use kale_syntax::ast::Block;

pub fn encode(block: &Block) -> Vec<u8> {
    let mut encoder = Encoder::new();
    encoder.encode(block);
    encoder.into_bytes()
}

pub fn decode(bytes: &[u8]) -> Result<Block> {
    Decoder::new(bytes).decode()
}
