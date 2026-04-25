pub(super) struct Cursor<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.cursor >= self.input.len()
    }

    pub(super) fn peek_ahead(&self, n: usize) -> Option<char> {
        self.input[self.cursor + n..].chars().next()
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.peek_ahead(0)
    }

    pub(super) fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.cursor += c.len_utf8();
        Some(c)
    }

    pub(super) fn advance_by(&mut self, n: usize) -> &str {
        let start = self.cursor;
        for _ in 0..n {
            if self.advance().is_none() { break; }
        }
        &self.input[start..self.cursor]
    }

    pub(super) fn advance_while<F>(&mut self, mut f: F) -> &str
    where
        F: FnMut(char) -> bool,
    {
        let start = self.cursor;
        while matches!(self.peek(), Some(c) if f(c)) {
            self.advance();
        }
        &self.input[start..self.cursor]
    }
}
