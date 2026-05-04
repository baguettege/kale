mod decode;
mod encode;
mod error;
mod tag;
mod file;

pub use error::{Error, Result};
pub use file::Kast;

use crate::decode::Decoder;
use crate::encode::Encoder;

pub fn encode(kast: &Kast) -> Vec<u8> {
    let mut encoder = Encoder::new();
    encoder
        .encode(&kast.source)
        .encode(&kast.program);
    encoder.into_bytes()
}

pub fn decode(bytes: &[u8]) -> Result<Kast> {
    let mut decoder = Decoder::new(bytes);
    let source = decoder.decode::<String>()?;
    let program = decoder.decode()?;
    Ok(Kast::new(source, program))
}
