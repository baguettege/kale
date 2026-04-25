use crate::ast::{Expr, Ident};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Module(Module),
    FnDef(FnDef),
    Assign(Assign),
    If(If),
    While(While),
    Return(Return),
}

#[derive(Debug, Clone)]
pub struct Block(pub Vec<Stmt>);

node! {
    Module {
        ident: Ident,
        body: Block,
    }
}

node! {
    FnDef {
        ident: Ident,
        params: Vec<Ident>,
        body: Block,
    }
}

node! {
    Assign {
        target: Expr,
        value: Expr,
    }
}

node! {
    If {
        cond: Expr,
        then_branch: Block,
        else_branch: Option<Block>,
    }
}

node! {
    While {
        cond: Expr,
        body: Block,
    }
}

node! {
    Return {
        value: Expr,
    }
}

impl_from!(Stmt => Expr, Module, FnDef, Assign, If, While, Return);
