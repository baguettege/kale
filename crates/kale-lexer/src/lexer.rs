use crate::lexer::cursor::Cursor;
use crate::Result;
use kale_syntax::token::TokenStream;

mod cursor;
mod token;
mod scan;

pub(crate) struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self { cursor: Cursor::new(input) }
    }

    pub(crate) fn tokenize(mut self) -> Result<TokenStream> {
        let mut tokens = Vec::new();

        loop {
            self.cursor.advance_while(char::is_whitespace);
            if self.cursor.is_at_end() { break; }
            tokens.push(self.scan_token()?);
        }

        Ok(TokenStream(tokens))
    }
}
