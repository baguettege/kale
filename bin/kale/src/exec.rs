use crate::{compile, run};
use std::path::Path;
use std::{fs, io};

pub(crate) fn exec<P: AsRef<Path>>(source: P) -> io::Result<()> {
    let code = fs::read_to_string(source)?;
    let kast = compile::compile_or_die(&code);
    run::run_or_die(&kast);
    Ok(())
}
