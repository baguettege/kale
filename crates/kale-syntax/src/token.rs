use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Nil,
    Num(f64),
    Bool(bool),
    Str(String),

    Ident(String),
    Assign,

    Star,
    Slash,
    Plus,
    Minus,

    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Is,

    Not,
    And,
    Or,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    Comma,
    Semicolon,
    Dot,

    Module,
    Fn,
    Return,
    If,
    Else,
    While,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Num(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Str(s) => write!(f, "\"{s}\""),
            Self::Ident(s) => write!(f, "{s}"),
            Self::Assign => write!(f, "="),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::EqEq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
            Self::Is => write!(f, "is"),
            Self::Not => write!(f, "not"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::LBrack => write!(f, "["),
            Self::RBrack => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Semicolon => write!(f, ";"),
            Self::Dot => write!(f, "."),
            Self::Module => write!(f, "module"),
            Self::Fn => write!(f, "fn"),
            Self::Return => write!(f, "return"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::While => write!(f, "while"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenStream(pub Vec<Token>);

impl fmt::Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements = self.0
            .iter()
            .map(|tok| format!("{tok}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{elements}]")
    }
}
