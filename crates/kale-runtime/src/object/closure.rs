use std::fmt;
use kale_syntax::ast::{Block, Ident};
use crate::env::Env;
use crate::object::Builtin;

#[derive(Debug, Clone)]
pub struct Closure {
    params: Vec<Ident>,
    body: Block,
    pub env: Env,
}

impl Closure {
    pub fn new(params: Vec<Ident>, body: Block, env: Env) -> Self {
        Self { params, body, env }
    }
    
    pub fn params(&self) -> &[Ident] {
        &self.params
    }
    
    pub fn body(&self) -> &Block {
        &self.body
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
        write!(f, "fn({})", self.params.join(", "))
    }
}

