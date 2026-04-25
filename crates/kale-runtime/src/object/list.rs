use std::fmt;
use crate::object::{Builtin, Object};

#[derive(Debug, Clone)]
pub struct List(Vec<Object>);

impl List {
    pub fn new(elements: Vec<Object>) -> Self {
        Self(elements)
    }

    pub fn elements(&self) -> &[Object] {
        &self.0
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
            .map(|elem| format!("{elem}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{params}]")
    }
}

