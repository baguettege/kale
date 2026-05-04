pub use crate::parser::Error;
use crate::parser;
use crate::codec::Kast;

pub fn compile(source: &str) -> Result<Kast, Error> {
    let program = parser::parse(source)?;
    Ok(Kast::new(source, program))
}
