use std::collections::HashMap;
use kale_syntax::ast::Ident;
use crate::object::{Immutable, Method, Object};

#[derive(Debug)]
pub struct StructDef {
    pub(super) fields: Vec<Ident>,
    pub(super) methods: HashMap<Ident, Method>,
}

#[derive(Debug)]
pub struct Struct {
    pub(super) def: Immutable<StructDef>,
    pub(super) fields: HashMap<Ident, Object>,
}

impl StructDef {
    pub fn new(fields: Vec<Ident>, methods: HashMap<Ident, Method>) -> Self {
        Self { fields, methods }
    }

    pub fn fields(&self) -> &[Ident] {
        &self.fields
    }

    pub fn method(&self, ident: &Ident) -> Option<Method> {
        self.methods.get(ident).cloned()
    }
}

impl Struct {
    pub fn new(def: Immutable<StructDef>, fields: HashMap<Ident, Object>) -> Self {
        Self { def, fields }
    }

    pub fn def(&self) -> &StructDef {
        &self.def
    }

    pub fn get(&self, ident: &Ident) -> Option<Object> {
        self.fields.get(ident).cloned()
    }

    pub fn set(&mut self, ident: &Ident, object: impl Into<Object>) -> Option<()> {
        self.fields
            .get_mut(ident)
            .map(|obj| *obj = object.into())
    }
}
