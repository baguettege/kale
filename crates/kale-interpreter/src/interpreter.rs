mod common;
mod flow;
mod expr;
mod stmt;
mod runtime;
mod call;

use kale_runtime::env::{Env, Globals};
use kale_syntax::ast::Program;
use kale_syntax::span::Span;
use crate::registry::{Loader, Registry};
use crate::Result;
use crate::interpreter::flow::Signal;

pub(crate) struct Interpreter {
    env: Env,
    // the current span being evaluated
    // stored as a field, so `Interpreter::error` always has access
    span: Span,
}

impl Interpreter {
    pub(crate) fn new(loaders: &[Loader]) -> Self {
        let mut globals = Globals::new();
        let mut setup = Registry::new(&mut globals);
        
        for init in loaders {
            init(&mut setup);
        }
        
        Self {
            env: Env::new(globals),
            span: Span::new(0, 0),
        }
    }

    pub(crate) fn run(mut self, program: &Program) -> Result<()> {
        self.eval(program)
    }

    fn eval(&mut self, program: &Program) -> Result<()> {
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
