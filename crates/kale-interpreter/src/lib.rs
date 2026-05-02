mod interpreter;
mod setup;

use crate::interpreter::Interpreter;
use kale_syntax::ast::Program;

pub use setup::{Init, Setup};

pub fn run(program: &Program, inits: &[Init]) -> kale_runtime::Result<()> {
    Interpreter::new(inits).run(program)
}
