use crate::error::Error;
use crate::lexer::{token, Lexer};
use crate::ErrorKind;
use crate::Result;
use kale_syntax::span::Span;
use kale_syntax::token::{Token, TokenKind};

type RawResult<T> = std::result::Result<T, ErrorKind>;

impl Lexer<'_> {
    pub(super) fn scan_token(&mut self) -> Result<Token> {
        let start = self.cursor.offset();

        let result = self.scan_kind();
        let span = Span::new(start, self.cursor.offset());

        match result {
            Ok(kind) => Ok(Token::new(span, kind)),
            Err(e) => Err(Error::new(span, e)),
        }
    }

    fn scan_kind(&mut self) -> RawResult<TokenKind> {
        match self.cursor.peek().ok_or(ErrorKind::UnexpectedEof)? {
            c if c.is_ascii_digit() => self.scan_num(),
            '\'' => self.scan_char(),
            '"' => self.scan_str(),
            c if c.is_ascii_alphabetic() || c == '_' => Ok(self.scan_ident()),
            c => token::punct(c)
                .map(|tok| { self.cursor.advance(); tok })
                .or_else(|| token::op(&mut self.cursor))
                .ok_or(ErrorKind::UnexpectedChar(c))
        }
    }

    fn scan_num(&mut self) -> RawResult<TokenKind> {
        let s = self.cursor.consume(|cursor| {
            // consume the 1st part of the number
            cursor.advance_while(|c| c.is_ascii_digit());

            if cursor.peek() == Some('.') &&
                cursor.peek_ahead(1).is_some_and(|c| c.is_ascii_digit())
            {
                cursor.advance(); // consume the `.`
                // consume the 2nd part of the number
                cursor.advance_while(|c| c.is_ascii_digit());
            }
        });

        s.parse::<f64>()
            .map(TokenKind::Num)
            .map_err(|_| ErrorKind::InvalidNum(s.to_string()))
    }

    fn scan_char(&mut self) -> RawResult<TokenKind> {
        self.expect('\'')?;
        let c = self.cursor.advance().ok_or(ErrorKind::UnexpectedEof)?;
        self.expect('\'')?;
        Ok(TokenKind::Char(c))
    }

    fn scan_str(&mut self) -> RawResult<TokenKind> {
        self.expect('"')?;
        let s = self.cursor.advance_while(
            |c| c != '"').to_string();
        self.expect('"')?;
        Ok(TokenKind::Str(s))
    }

    fn scan_ident(&mut self) -> TokenKind {
        let s = self.cursor.advance_while(
            |c| c.is_ascii_alphanumeric() || c == '_');
        token::keyword(s).unwrap_or_else(
            || TokenKind::Ident(s.to_string()))
    }
}

impl Lexer<'_> {
    fn expect(&mut self, c: char) -> RawResult<()> {
        match self.cursor.advance() {
            Some(ch) if ch == c => Ok(()),
            Some(ch) => Err(ErrorKind::UnexpectedChar(ch)),
            None => Err(ErrorKind::UnexpectedEof),
        }
    }
}
