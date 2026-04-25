use kale_syntax::token::Token;
use crate::lexer::{token, Lexer};
use crate::{Result, Error};

impl Lexer<'_> {
    pub(super) fn scan_token(&mut self) -> Result<Token> {
        match self.cursor.peek().ok_or(Error::UnexpectedEof)? {
            c if c.is_ascii_digit() => self.scan_num(),
            '"' => self.scan_str(),
            c if c.is_ascii_alphabetic() || c == '_' => Ok(self.scan_ident()),
            c => token::punct(c)
                .map(|tok| { self.cursor.advance(); tok })
                .or_else(|| token::op(&mut self.cursor))
                .ok_or(Error::UnexpectedChar(c))
        }
    }

    fn scan_num(&mut self) -> Result<Token> {
        let s = self.cursor.advance_while(
            |c| c.is_ascii_digit() || c == '.');
        s.parse::<f64>()
            .map(Token::Num)
            .map_err(|_| Error::InvalidNum(s.to_string()))
    }

    fn scan_str(&mut self) -> Result<Token> {
        self.expect('"')?;
        let s = self.cursor.advance_while(
            |c| c != '"').to_string();
        self.expect('"')?;
        Ok(Token::Str(s))
    }

    fn scan_ident(&mut self) -> Token {
        let s = self.cursor.advance_while(
            |c| c.is_ascii_alphanumeric() || c == '_');
        token::keyword(s).unwrap_or_else(
            || Token::Ident(s.to_string()))
    }
}

impl Lexer<'_> {
    fn expect(&mut self, c: char) -> Result<()> {
        match self.cursor.advance() {
            Some(ch) if ch == c => Ok(()),
            Some(ch) => Err(Error::UnexpectedChar(ch)),
            None => Err(Error::UnexpectedEof),
        }
    }
}
