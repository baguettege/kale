mod decode;
mod encode;
mod error;
mod tag;

pub use error::{Error, Result};
use crate::decode::Decoder;
use crate::encode::Encoder;
use kale_syntax::ast::Program;

pub fn encode(program: &Program) -> Vec<u8> {
    let mut encoder = Encoder::new();
    encoder.encode(program);
    encoder.into_bytes()
}

pub fn decode(bytes: &[u8]) -> Result<Program> {
    Decoder::new(bytes).decode()
}
