use kale_syntax::ast::Program;
use crate::object::Object;
use crate::Result;

pub trait Runtime {
    fn call(&mut self, callee: Object, args: &[Object]) -> Result<Object>;
    fn eval(&mut self, program: &Program) -> Result<()>;
}

pub struct Ctx<'a>(&'a mut dyn Runtime);

impl<'a> Ctx<'a> {
    pub fn new<T: Runtime>(runtime: &'a mut T) -> Self {
        Self(runtime)
    }

    pub fn call(&mut self, callee: impl Into<Object>, args: &[Object]) -> Result<Object> {
        self.0.call(callee.into(), args)
    }

    pub fn eval(&mut self, program: &Program) -> Result<()> {
        self.0.eval(program)
    }
}
