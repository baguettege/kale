use crate::ast::{BinOp, Block, Ident, Literal, UnOp};
use crate::span::Spanned;

pub type Expr = Spanned<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Literal(Literal),
    Ident(Ident),
    
    Call(Call),
    Binary(Binary),
    Unary(Unary),
    List(List),
    Closure(Closure),
    Member(Member),
}

node! {
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    }
}

node! {
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    }
}

node! {
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    }
}

node! {
    List {
        elements: Vec<Expr>,
    }
}

node! {
    Closure {
        params: Vec<Ident>,
        body: Block,
    }
}

node! {
    Member {
        object: Box<Expr>,
        property: Ident,
    }
}

impl_from!(ExprKind => Literal, Ident, Call, Binary, Unary, List, Closure, Member);
