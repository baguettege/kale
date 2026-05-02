mod common;
mod flow;
mod expr;
mod stmt;
mod runtime;
mod call;

use kale_runtime::env::{Env, Globals};
use kale_syntax::ast::Program;
use crate::setup::{Init, Setup};
use crate::interpreter::flow::Signal;

pub(crate) struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub(crate) fn new(inits: &[Init]) -> Self {
        let mut globals = Globals::new();
        let mut setup = Setup::new(&mut globals);
        
        for init in inits {
            init(&mut setup);
        }
        
        Self { env: Env::new(globals) }
    }

    pub(crate) fn run(mut self, program: &Program) -> kale_runtime::Result<()> {
        self.eval(program)
    }

    fn eval(&mut self, program: &Program) -> kale_runtime::Result<()> {
        let result = self.with_env(
            self.env.isolate(),
            |this| this.eval_block(&program.0),
        );

        match result {
            Ok(()) | Err(Signal::Return(_)) => Ok(()),
            Err(Signal::Error(e)) => Err(e),
        }
    }
}
