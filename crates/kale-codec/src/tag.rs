use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub(crate) enum AstTag {
    Ident,
    BinOp,
    UnOp,
    Literal,

    Call,
    Binary,
    Unary,
    List,
    Closure,
    Member,

    Module,
    Struct,
    FnDef,
    Let,
    Assign,
    If,
    While,
    Return,
    Raise,

    Expr,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub(crate) enum LiteralTag {
    Nil,
    Num,
    Bool,
    Char,
    Str,
}
