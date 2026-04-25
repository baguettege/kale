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
            Token::Nil => write!(f, "nil"),
            Token::Num(n) => write!(f, "{n}"),
            Token::Bool(b) => write!(f, "{b}"),
            Token::Str(s) => write!(f, "\"{s}\""),
            Token::Ident(s) => write!(f, "{s}"),
            Token::Assign => write!(f, "="),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::EqEq => write!(f, "=="),
            Token::Ne => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Le => write!(f, "<="),
            Token::Gt => write!(f, ">"),
            Token::Ge => write!(f, ">="),
            Token::Not => write!(f, "not"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBrack => write!(f, "["),
            Token::RBrack => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Dot => write!(f, "."),
            Token::Module => write!(f, "module"),
            Token::Fn => write!(f, "fn"),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
        }
    }
}
