use kale_syntax::ast::{Assign, Block, FnDef, If, Module, Return, Stmt, While};
use crate::encode::{Encode, Encoder};
use crate::tag::AstTag;

impl Encode for Stmt {
    fn encode(&self, encoder: &mut Encoder) {
        fn tagged<T: Encode>(encoder: &mut Encoder, tag: AstTag, value: &T) {
            encoder
                .encode(&tag)
                .encode(value);
        }

        match self {
            Stmt::Expr(node) => tagged(encoder, AstTag::Expr, node),
            Stmt::Module(node) => tagged(encoder, AstTag::Module, node),
            Stmt::FnDef(node) => tagged(encoder, AstTag::FnDef, node),
            Stmt::Assign(node) => tagged(encoder, AstTag::Assign, node),
            Stmt::If(node) => tagged(encoder, AstTag::If, node),
            Stmt::While(node) => tagged(encoder, AstTag::While, node),
            Stmt::Return(node) => tagged(encoder, AstTag::Return, node),
        }
    }
}

impl Encode for Block {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&self.0);
    }
}

impl Encode for Module {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.ident)
            .encode(&self.body);
    }
}

impl Encode for FnDef {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.ident)
            .encode(&self.params)
            .encode(&self.body);
    }
}

impl Encode for Assign {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.target)
            .encode(&self.value);
    }
}

impl Encode for If {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.cond)
            .encode(&self.then_branch)
            .encode(&self.else_branch);
    }
}

impl Encode for While {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.cond)
            .encode(&self.body);
    }
}

impl Encode for Return {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&self.value);
    }
}
