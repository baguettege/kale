use crate::object::Object;
use crate::{Result, Error};
use std::collections::HashMap;
use kale_syntax::ast::Ident;

#[derive(Debug, Clone)]
struct Scope(HashMap<Ident, Object>);

impl Scope {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn define(&mut self, ident: impl Into<Ident>, object: Object) {
        self.0.insert(ident.into(), object);
    }

    fn lookup(&self, ident: &Ident) -> Option<Object> {
        self.0.get(ident).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Env(Vec<Scope>);

impl Env {
    pub fn new() -> Self {
        Self(vec![Scope::new()])
    }

    pub fn enter_scope(&mut self) {
        self.0.push(Scope::new());
    }

    pub fn exit_scope(&mut self) -> Result<HashMap<Ident, Object>> {
        if self.0.len() > 1 {
            Ok(self.0.pop().unwrap().0)
        } else {
            Err(Error::ScopeUnderflow)
        }
    }

    pub fn define(&mut self, ident: impl Into<Ident>, object: Object) {
        self.0
            .last_mut()
            .expect("bug: scope stack underflow")
            .define(ident, object);
    }

    pub fn lookup(&self, ident: &Ident) -> Result<Object> {
        self.0
            .iter()
            .rev()
            .find_map(|scope| scope.lookup(ident))
            .ok_or_else(|| Error::UndefinedVariable(ident.clone()))
    }
}
