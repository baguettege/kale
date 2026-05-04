pub(super) struct Cursor<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(input: &'a str) -> Self {
        Self { input, offset: 0 }
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.offset >= self.input.len()
    }

    pub(super) fn offset(&self) -> usize {
        self.offset
    }

    pub(super) fn peek_ahead(&self, n: usize) -> Option<char> {
        self.input[self.offset + n..].chars().next()
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.peek_ahead(0)
    }

    pub(super) fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.offset += c.len_utf8();
        Some(c)
    }

    pub(super) fn consume<F>(&mut self, f: F) -> &str
    where
        F: FnOnce(&mut Self),
    {
        let start = self.offset;
        f(self);
        &self.input[start..self.offset]
    }

    pub(super) fn advance_by(&mut self, n: usize) -> &str {
        self.consume(|this| {
            for _ in 0..n {
                if this.advance().is_none() {
                    break;
                }
            }
        })
    }

    pub(super) fn advance_while<F>(&mut self, mut f: F) -> &str
    where
        F: FnMut(char) -> bool,
    {
        self.consume(|this| {
            while matches!(this.peek(), Some(c) if f(c)) {
                this.advance();
            }
        })
    }
}
