mod flow;
mod stmt;
mod expr;
mod common;

use crate::interpreter::flow::Outcome;
use kale_runtime::builtin::Lib;
use kale_runtime::env::Env;
use kale_runtime::object::Module;
use kale_syntax::ast::Block;

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new(libs: &[Lib]) -> Self {
        let mut env = Env::new();

        for lib in libs {
            let mut module = Module::new();

            for builtin in lib.builtins {
                let ident = builtin.ident.to_string();
                module.define(ident, builtin.into());
            }

            env.define(lib.ident.to_string(), module.into());
        }

        Self { env }
    }

    pub fn run(mut self, block: &Block) -> kale_runtime::Result<()> {
        match self.eval_block(block) {
            Ok(_) | Err(Outcome::Return(_)) => Ok(()),
            Err(Outcome::Error(e)) => Err(e),
        }
    }
}
