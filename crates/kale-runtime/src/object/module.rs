use std::collections::HashMap;
use kale_syntax::ast::Ident;
use crate::env::{Globals, Scope};
use crate::object::Object;

#[derive(Debug)]
pub struct Module(pub(super) Scope);

impl Module {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn define(&mut self, ident: impl Into<Ident>, object: impl Into<Object>) {
        self.0.insert(ident.into(), object.into());
    }

    pub fn set(&mut self, ident: impl Into<Ident>, object: impl Into<Object>) -> Option<()> {
        let ident = ident.into();
        self.0.get_mut(&ident).map(|old| *old = object.into())
    }

    pub fn lookup(&self, ident: &Ident) -> Option<Object> {
        self.0.get(ident).cloned()
    }
}

impl From<Scope> for Module {
    fn from(scope: Scope) -> Self {
        Self(scope)
    }
}

impl From<Globals> for Module {
    fn from(globals: Globals) -> Self {
        Self(globals.into())
    }
}
