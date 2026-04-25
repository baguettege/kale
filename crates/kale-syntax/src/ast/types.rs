pub type Ident = String;

#[derive(Debug, Copy, Clone)]
pub enum BinOp {
    Mul,
    Div,

    Add,
    Sub,

    Lt,
    Le,
    Gt,
    Ge,

    Eq,
    Ne,

    And,

    Or,
}

#[derive(Debug, Copy, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Num(f64),
    Bool(bool),
    Str(String),
}
