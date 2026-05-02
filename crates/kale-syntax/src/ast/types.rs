use num_enum::TryFromPrimitive;

pub type Ident = String;

#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum BinOp {
    Mul,
    Div,
    Mod,

    Add,
    Sub,

    Lt,
    Le,
    Gt,
    Ge,

    Eq,
    Ne,
    Is,

    And,

    Or,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Num(f64),
    Bool(bool),
    Char(char),
    Str(String),
}
