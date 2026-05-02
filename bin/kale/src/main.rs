mod error;

use crate::error::{Error, Result};
use kale_api::{codec, interpreter};
use std::path::PathBuf;
use std::{env, fs, process};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let name = env::args().nth(1).ok_or(Error::NoFile)?;
    let file = PathBuf::from(&name).with_extension("kast");
    let encoded = fs::read(&file)?;

    let program = codec::decode(&encoded)?;
    interpreter::run(&program, &[kale_stdlib::INIT])?;

    Ok(())
}
