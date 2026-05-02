mod error;

use crate::error::Error;
use error::Result;
use kale_api::{codec, parser};
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
    let in_file = PathBuf::from(&name).with_extension("kale");
    let src = fs::read_to_string(&in_file)?;

    let program = parser::parse(&src)?;
    let encoded = codec::encode(&program);

    let out_file = in_file.with_extension("kast");
    fs::write(&out_file, &encoded)?;

    Ok(())
}
