use std::fmt;
use std::fmt::Formatter;
use crate::object::{Builtin, Object};

#[derive(Debug, Clone)]
pub enum Method {
    Builtin(&'static Builtin),
    // `Frozen<Closure>` for kale-defined methods
}

#[derive(Debug, Clone)]
pub struct BoundMethod {
    pub receiver: Object,
    pub method: Method,
}

impl BoundMethod {
    pub fn new(receiver: Object, method: Method) -> Self {
        Self { receiver, method }
    }
}

impl super::Type for BoundMethod {
    fn type_name() -> &'static str {
        "method"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for BoundMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<method>")
    }
}
