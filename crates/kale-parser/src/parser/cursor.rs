use kale_syntax::span::Span;
use kale_syntax::token::{Token, TokenKind};

pub(crate) struct Cursor<'a> {
    tokens: &'a [Token],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, offset: 0 }
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.offset >= self.tokens.len()
    }

    pub(crate) fn span(&self) -> Span {
        self.peek()
            .or_else(|| self.tokens.last())
            .map(Token::span)
            .unwrap_or(Span::new(0, 0))
    }

    pub(crate) fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.offset + n)
    }

    pub(crate) fn peek(&self) -> Option<&Token> {
        self.peek_ahead(0)
    }

    pub(crate) fn prev(&self) -> Option<&Token> {
        self.offset.checked_sub(1)
            .and_then(|i| self.tokens.get(i))
    }

    pub(crate) fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.offset)?;
        self.offset += 1;
        Some(token)
    }

    pub(crate) fn consume_if(&mut self, kind: TokenKind) -> bool {
        if self.peek().map(Token::inner) == Some(&kind) {
            self.advance();
            true
        } else {
            false
        }
    }
}
