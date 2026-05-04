use std::fs;
use std::path::Path;
use kale_api::codec;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Codec(#[from] codec::Error),
}

pub(crate) fn show<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let bytes = fs::read(path)?;
    let kast = codec::decode(&bytes)?;
    println!("{}", kast.source);
    Ok(())
}
