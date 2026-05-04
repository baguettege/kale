use kale_syntax::token::TokenKind;
use crate::lexer::cursor::Cursor;

pub(super) fn keyword(s: &str) -> Option<TokenKind> {
    Some(match s {
        "nil" => TokenKind::Nil,
        "true" => TokenKind::Bool(true),
        "false" => TokenKind::Bool(false),

        "is" => TokenKind::Is,

        "not" => TokenKind::Not,
        "and" => TokenKind::And,
        "or" => TokenKind::Or,

        "fn" => TokenKind::Fn,
        "return" => TokenKind::Return,
        "let" => TokenKind::Let,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "module" => TokenKind::Module,
        "struct" => TokenKind::Struct,
        "raise" => TokenKind::Raise,

        _ => return None,
    })
}

pub(super) fn punct(c: char) -> Option<TokenKind> {
    Some(match c {
        '(' => TokenKind::LParen,
        ')' => TokenKind::RParen,
        '{' => TokenKind::LBrace,
        '}' => TokenKind::RBrace,
        '[' => TokenKind::LBrack,
        ']' => TokenKind::RBrack,

        ',' => TokenKind::Comma,
        ';' => TokenKind::Semicolon,
        '.' => TokenKind::Dot,
        '|' => TokenKind::Pipe,

        _ => return None,
    })
}

pub(super) fn op(cursor: &mut Cursor) -> Option<TokenKind> {
    let (token, n) = match (cursor.peek()?, cursor.peek_ahead(1)) {
        ('*', _) => (TokenKind::Star, 1),
        ('/', _) => (TokenKind::Slash, 1),
        ('%', _) => (TokenKind::Percent, 1),
        ('+', _) => (TokenKind::Plus, 1),
        ('-', _) => (TokenKind::Minus, 1),

        ('=', Some('=')) => (TokenKind::EqEq, 2),
        ('=', _) => (TokenKind::Assign, 1),

        ('!', Some('=')) => (TokenKind::Ne, 2),

        ('<', Some('=')) => (TokenKind::Le, 2),
        ('<', _) => (TokenKind::Lt, 1),

        ('>', Some('=')) => (TokenKind::Ge, 2),
        ('>', _) => (TokenKind::Gt, 1),

        _ => return None,
    };

    cursor.advance_by(n);
    Some(token)
}
