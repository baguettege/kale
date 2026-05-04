mod interpreter;
mod registry;
mod error;

use crate::interpreter::Interpreter;
use kale_syntax::ast::Program;

pub use registry::{Loader, Registry};
pub use error::{Error, Result};

pub fn run(program: &Program, loaders: &[Loader]) -> Result<()> {
    Interpreter::new(loaders).run(program)
}
