use std::{fs, io};
use std::path::Path;
use kale_api::{codec, compiler};
use kale_api::codec::Kast;
use crate::die;

pub(crate) fn compile<P: AsRef<Path>>(source: P) -> io::Result<()> {
    let source = source.as_ref();
    let output = source.with_extension("kast");

    let code = fs::read_to_string(source)?;
    let kast = compile_or_die(&code);

    let bytes = codec::encode(&kast);
    fs::write(output, bytes)?;

    Ok(())
}

pub(crate) fn compile_or_die(source: &str) -> Kast {
    die::or_die(&source, compiler::compile(&source))
}
