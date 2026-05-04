use kale_runtime::env::Globals;
use kale_runtime::object::{Module, Object};
use kale_syntax::ast::Ident;

pub type Loader = fn(&mut Registry);

pub struct Registry<'a>(&'a mut Globals);

impl<'a> Registry<'a> {
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
        f: impl FnOnce(Registry),
    ) -> &mut Self {
        let mut globals = Globals::new();
        f(Registry::new(&mut globals));

        let module = Module::from(globals);
        self.define(ident, module);
        self
    }
}
