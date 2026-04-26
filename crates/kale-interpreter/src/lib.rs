use kale_runtime::builtin::Lib;
use kale_runtime::Result;
use kale_syntax::ast::Block;
use crate::interpreter::Interpreter;

mod interpreter;

pub fn run(block: &Block, libs: &[&Lib]) -> Result<()> {
    Interpreter::new(libs).run(block)
}
