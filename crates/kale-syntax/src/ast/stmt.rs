use crate::ast::{Expr, Ident};
use crate::span::Spanned;

pub type Stmt = Spanned<StmtKind>;
pub type Block = Vec<Stmt>;

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr(Expr),

    Module(Module),
    Struct(Struct),
    FnDef(FnDef),
    Let(Let),
    Assign(Assign),
    If(If),
    While(While),
    Return(Return),
    Raise(Raise),
}

node! {
    Module {
        ident: Ident,
        body: Block,
    }
}

node! {
    Struct {
        ident: Ident,
        fields: Vec<Ident>,
        methods: Vec<FnDef>,
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
    Let {
        ident: Ident,
        init: Expr,
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

node! {
    Raise {
        value: Expr,
    }
}

impl_from!(StmtKind => Expr, Module, Struct, FnDef, Let, Assign, If, While, Return, Raise);
