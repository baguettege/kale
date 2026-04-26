use kale_syntax::token::Token;

pub(crate) struct Cursor<'a> {
    tokens: &'a [Token],
    cursor: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    pub(crate) fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.cursor + n)
    }

    pub(crate) fn peek(&self) -> Option<&Token> {
        self.peek_ahead(0)
    }

    pub(crate) fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;
        Some(token)
    }

    pub(crate) fn consume_if(&mut self, token: Token) -> bool {
        if self.peek() == Some(&token) {
            self.advance();
            true
        } else {
            false
        }
    }
}
