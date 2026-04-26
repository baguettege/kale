use crate::object::Object;
use crate::{Result, Error};
use std::collections::HashMap;
use kale_syntax::ast::Ident;

type Scope = HashMap<Ident, Object>;

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
            Ok(self.0.pop().unwrap())
        } else {
            panic!("bug: scope stack underflow");
        }
    }

    pub fn define(&mut self, ident: impl Into<Ident>, object: Object) {
        let ident = ident.into();

        let scope = self.0
            .iter_mut()
            .rev()
            .find(|scope| scope.contains_key(&ident));

        match scope {
            None => self.0.last_mut().unwrap(),
            Some(scope) => scope,
        }.insert(ident, object);
    }

    pub fn lookup(&self, ident: &Ident) -> Result<Object> {
        self.0
            .iter()
            .rev()
            .find_map(|scope| scope.get(ident))
            .cloned()
            .ok_or_else(|| Error::UndefinedVariable(ident.clone()))
    }
}
