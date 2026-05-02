use crate::env::Env;
use kale_syntax::ast::{Block, Ident};

#[derive(Debug)]
pub struct Closure {
    pub(super) params: Vec<Ident>,
    pub(super) body: Block,
    pub(super) env: Env,
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

    pub fn env(&self) -> &Env {
        &self.env
    }
}
