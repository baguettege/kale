use kale_syntax::token::Token;
use crate::lexer::cursor::Cursor;

pub(super) fn keyword(s: &str) -> Option<Token> {
    Some(match s {
        "nil" => Token::Nil,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),

        "is" => Token::Is,

        "not" => Token::Not,
        "and" => Token::And,
        "or" => Token::Or,

        "module" => Token::Module,
        "fn" => Token::Fn,
        "return" => Token::Return,
        "if" => Token::If,
        "else" => Token::Else,
        "while" => Token::While,

        _ => return None,
    })
}

pub(super) fn punct(c: char) -> Option<Token> {
    Some(match c {
        '(' => Token::LParen,
        ')' => Token::RParen,
        '{' => Token::LBrace,
        '}' => Token::RBrace,
        '[' => Token::LBrack,
        ']' => Token::RBrack,
        ',' => Token::Comma,
        ';' => Token::Semicolon,
        '.' => Token::Dot,
        _ => return None,
    })
}

pub(super) fn op(cursor: &mut Cursor) -> Option<Token> {
    let (token, n) = match (cursor.peek()?, cursor.peek_ahead(1)) {
        ('+', _) => (Token::Plus, 1),
        ('-', _) => (Token::Minus, 1),
        ('*', _) => (Token::Star, 1),
        ('/', _) => (Token::Slash, 1),

        ('=', Some('=')) => (Token::EqEq, 2),
        ('=', _) => (Token::Assign, 1),

        ('!', Some('=')) => (Token::Ne, 2),

        ('<', Some('=')) => (Token::Le, 2),
        ('<', _) => (Token::Lt, 1),

        ('>', Some('=')) => (Token::Ge, 2),
        ('>', _) => (Token::Gt, 1),

        _ => return None,
    };

    cursor.advance_by(n);
    Some(token)
}
