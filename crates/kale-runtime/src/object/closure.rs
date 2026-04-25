use std::fmt;
use kale_syntax::ast::{Block, Ident};
use crate::env::Env;
use crate::object::Builtin;

#[derive(Debug, Clone)]
pub struct Closure {
    pub params: Vec<Ident>,
    pub body: Block,
    pub env: Env,
}

impl Closure {
    pub fn new(params: Vec<Ident>, body: Block, env: Env) -> Self {
        Self { params, body, env }
    }
}

impl super::Type for Closure {
    fn type_name() -> &'static str {
        "closure"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.params
            .iter()
            .map(Ident::as_str)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "fn({params})")
    }
}

