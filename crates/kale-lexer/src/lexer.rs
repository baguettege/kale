use kale_syntax::token::Token;
use crate::lexer::cursor::Cursor;
use crate::Result;

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

    pub(crate) fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            self.skip_trivia();
            if self.cursor.is_at_end() { break; }
            tokens.push(self.scan_token()?);
        }

        Ok(tokens)
    }

    fn skip_trivia(&mut self) {
        loop {
            self.cursor.advance_while(char::is_whitespace);

            if self.cursor.peek() == Some('#') {
                self.cursor.advance_while(|c| c != '\n');
            } else {
                break;
            }
        }
    }
}
