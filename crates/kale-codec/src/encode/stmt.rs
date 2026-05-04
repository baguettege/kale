use kale_syntax::ast::{Assign, FnDef, If, Let, Module, Raise, Return, StmtKind, Struct, While};
use crate::encode::{Encode, Encoder};
use crate::tag::AstTag;

impl Encode for StmtKind {
    fn encode(&self, encoder: &mut Encoder) {
        fn tagged<T: Encode>(encoder: &mut Encoder, tag: AstTag, value: &T) {
            encoder
                .encode(&tag)
                .encode(value);
        }

        match self {
            Self::Expr(node) => tagged(encoder, AstTag::Expr, node),
            Self::Module(node) => tagged(encoder, AstTag::Module, node),
            Self::Struct(node) => tagged(encoder, AstTag::Struct, node),
            Self::FnDef(node) => tagged(encoder, AstTag::FnDef, node),
            Self::Let(node) => tagged(encoder, AstTag::Let, node),
            Self::Assign(node) => tagged(encoder, AstTag::Assign, node),
            Self::If(node) => tagged(encoder, AstTag::If, node),
            Self::While(node) => tagged(encoder, AstTag::While, node),
            Self::Return(node) => tagged(encoder, AstTag::Return, node),
            Self::Raise(node) => tagged(encoder, AstTag::Raise, node),
        }
    }
}

impl Encode for Module {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.ident)
            .encode(&self.body);
    }
}

impl Encode for Struct {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.ident)
            .encode(&self.fields)
            .encode(&self.methods);
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

impl Encode for Let {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.ident)
            .encode(&self.init);
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

impl Encode for Raise {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&self.value);
    }
}
