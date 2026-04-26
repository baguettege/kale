use std::fmt;
use crate::object::{Builtin, Object};

#[derive(Debug, Clone)]
pub struct List(Vec<Object>);

impl List {
    pub fn new(elements: Vec<Object>) -> Self {
        Self(elements)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, object: Object) {
        self.0.push(object);
    }

    pub fn pop(&mut self) -> Option<Object> {
        self.0.pop()
    }

    pub fn get(&self, index: usize) -> Option<Object> {
        self.0.get(index).cloned()
    }

    pub fn set(&mut self, index: usize, object: Object) -> Option<()> {
        if index < self.0.len() {
            self.0[index] = object;
            Some(())
        } else {
            None
        }
    }
}

impl super::Type for List {
    fn type_name() -> &'static str {
        "list"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.0
            .iter()
            .map(Object::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{params}]")
    }
}

