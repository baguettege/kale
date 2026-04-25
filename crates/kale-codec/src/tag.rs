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
    Index,

    Module,
    FnDef,
    Assign,
    If,
    While,
    Return,

    Expr,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub(crate) enum LiteralTag {
    Nil,
    Num,
    Bool,
    Str,
}
