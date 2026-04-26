use crate::object::{Builtin, Object};
use kale_syntax::ast::Ident;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Module(HashMap<Ident, Object>);

impl Module {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn define(&mut self, ident: impl Into<Ident>, object: Object) {
        self.0.insert(ident.into(), object);
    }

    pub fn lookup(&self, ident: &Ident) -> Option<Object> {
        self.0.get(ident).cloned()
    }
}

impl super::Type for Module {
    fn type_name() -> &'static str {
        "module"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl From<HashMap<Ident, Object>> for Module {
    fn from(bindings: HashMap<Ident, Object>) -> Self {
        Self(bindings)
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<module>")
    }
}

