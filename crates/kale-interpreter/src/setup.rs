use kale_runtime::env::Globals;
use kale_runtime::object::{Module, Object};
use kale_syntax::ast::Ident;

pub type Init = fn(&mut Setup);

pub struct Setup<'a>(&'a mut Globals);

impl<'a> Setup<'a> {
    pub(crate) fn new(globals: &'a mut Globals) -> Self {
        Self(globals)
    }

    pub fn define(
        &mut self,
        ident: impl Into<Ident>,
        object: impl Into<Object>,
    ) -> &mut Self {
        self.0.define(ident, object);
        self
    }

    pub fn module(
        &mut self,
        ident: impl Into<Ident>,
        f: impl FnOnce(Setup),
    ) -> &mut Self {
        let mut globals = Globals::new();
        f(Setup::new(&mut globals));

        let module = Module::from(globals);
        self.define(ident, module);
        self
    }
}
