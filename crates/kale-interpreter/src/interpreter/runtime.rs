use kale_runtime::ctx::Runtime;
use kale_runtime::object::Object;
use kale_syntax::ast::Program;
use crate::Error;
use crate::interpreter::flow::Signal;
use crate::interpreter::Interpreter;

impl Runtime for Interpreter {
    fn call(&mut self, callee: Object, args: &[Object]) -> kale_runtime::Result<Object> {
        match self.call(callee, args.to_vec()) {
            Ok(value) | Err(Signal::Return(value)) => Ok(value),
            Err(Signal::Error(e)) => Err(e.into_inner()),
        }
    }

    fn eval(&mut self, program: &Program) -> kale_runtime::Result<()> {
        self.eval(program).map_err(Error::into_inner)
    }
}
