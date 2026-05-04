use crate::die;
use kale_api::{codec, interpreter};
use std::fs;
use std::path::Path;
use kale_api::codec::Kast;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Codec(#[from] codec::Error),
}

pub(crate) fn run<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let bytes = fs::read(path)?;
    let kast = codec::decode(&bytes)?;
    run_or_die(&kast);
    Ok(())
}

pub(crate) fn run_or_die(kast: &Kast) {
    let result = interpreter::run(&kast.program, &[kale_stdlib::LOADER]);
    die::or_die(&kast.source, result);
}
