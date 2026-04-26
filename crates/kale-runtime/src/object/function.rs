use std::fmt;
use kale_syntax::ast::{Block, Ident};
use crate::object::Builtin;

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<Ident>,
    pub body: Block,
}

impl Function {
    pub fn new(params: Vec<Ident>, body: Block) -> Self {
        Self { params, body }
    }
}

impl super::Type for Function {
    fn type_name() -> &'static str {
        "function"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn({})", self.params.join(", "))
    }
}
