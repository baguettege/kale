use kale_syntax::ast::{Block, Ident};
use kale_syntax::token::Token;
use crate::parser::Parser;
use crate::{Error, Result};

impl Parser<'_> {
    pub(super) fn expect(&mut self, token: Token) -> Result<()> {
        match self.cursor.advance() {
            Some(tok) if tok == &token => Ok(()),
            Some(tok) => Err(Error::UnexpectedToken(tok.to_string())),
            None => Err(Error::UnexpectedEof),
        }
    }

    pub(super) fn parse_block(&mut self) -> Result<Block> {
        self.expect(Token::LBrace)?;
        let mut stmts = Vec::new();

        loop {
            match self.cursor.peek() {
                Some(Token::RBrace) => break,
                Some(_) => stmts.push(self.parse_stmt()?),
                None => return Err(Error::UnexpectedEof),
            }
        }

        self.expect(Token::RBrace)?;
        Ok(Block(stmts))
    }

    pub(super) fn parse_ident(&mut self) -> Result<Ident> {
        match self.cursor.advance() {
            Some(Token::Ident(s)) => Ok(s.clone()),
            Some(tok) => Err(Error::UnexpectedToken(tok.to_string())),
            None => Err(Error::UnexpectedEof),
        }
    }

    pub(super) fn parse_group<T, F>(
        &mut self,
        start: Token,
        end: Token,
        mut f: F,
    ) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        self.expect(start)?;
        let mut items = Vec::new();

        if self.cursor.consume_if(end.clone()) { return Ok(items); }

        loop {
            items.push(f(self)?);
            match self.cursor.peek() {
                Some(Token::Comma) => self.cursor.advance(),
                _ => break,
            };
        }

        self.expect(end)?;
        Ok(items)
    }
}
