use std::fmt;
use crate::span::Spanned;

pub type Token = Spanned<TokenKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Nil,
    Num(f64),
    Bool(bool),
    Char(char),
    Str(String),

    Ident(String),
    Assign,

    Star,
    Slash,
    Percent,
    Plus,
    Minus,

    Is,
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
    Pipe,

    Fn,
    Return,
    Let,
    If,
    Else,
    While,
    Module,
    Struct,
    Raise,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Num(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Char(c) => write!(f, "'{c}'"),
            Self::Str(s) => write!(f, "\"{s}\""),
            Self::Ident(i) => write!(f, "{i}"),

            Self::Assign => write!(f, "="),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),

            Self::Is => write!(f, "is"),
            Self::EqEq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),

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
            Self::Pipe => write!(f, "|"),

            Self::Fn => write!(f, "fn"),
            Self::Return => write!(f, "return"),
            Self::Let => write!(f, "let"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::While => write!(f, "while"),
            Self::Module => write!(f, "module"),
            Self::Struct => write!(f, "struct"),
            Self::Raise => write!(f, "raise"),
        }
    }
}
