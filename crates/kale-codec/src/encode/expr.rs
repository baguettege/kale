use kale_syntax::ast::{Binary, Call, Closure, Expr, Index, List, Member, Unary};
use crate::encode::{Encode, Encoder};
use crate::tag::AstTag;

impl Encode for Expr {
    fn encode(&self, encoder: &mut Encoder) {
        fn tagged<T: Encode>(encoder: &mut Encoder, tag: AstTag, value: &T) {
            encoder
                .encode(&tag)
                .encode(value);
        }

        match self {
            Self::Literal(lit) => tagged(encoder, AstTag::Literal, lit),
            Self::Ident(ident) => tagged(encoder, AstTag::Ident, ident),
            Self::Call(node) => tagged(encoder, AstTag::Call, node),
            Self::Binary(node) => tagged(encoder, AstTag::Binary, node),
            Self::Unary(node) => tagged(encoder, AstTag::Unary, node),
            Self::List(node) => tagged(encoder, AstTag::List, node),
            Self::Closure(node) => tagged(encoder, AstTag::Closure, node),
            Self::Member(node) => tagged(encoder, AstTag::Member, node),
            Self::Index(node) => tagged(encoder, AstTag::Index, node),
        }
    }
}

impl Encode for Box<Expr> {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(self.as_ref());
    }
}

impl Encode for Call {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.callee)
            .encode(&self.args);
    }
}

impl Encode for Binary {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.lhs)
            .encode(&self.op)
            .encode(&self.rhs);
    }
}

impl Encode for Unary {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.op)
            .encode(&self.expr);
    }
}

impl Encode for List {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&self.elements);
    }
}

impl Encode for Closure {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.params)
            .encode(&self.body);
    }
}

impl Encode for Member {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.object)
            .encode(&self.property);
    }
}

impl Encode for Index {
    fn encode(&self, encoder: &mut Encoder) {
        encoder
            .encode(&self.object)
            .encode(&self.index);
    }
}
