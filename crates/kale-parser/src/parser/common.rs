use kale_syntax::ast::{Block, Ident};
use kale_syntax::span::Span;
use kale_syntax::token::{Token, TokenKind};
use crate::parser::Parser;
use crate::{error, ErrorKind, Result};
use crate::error::Error;

impl Parser<'_> {
    pub(super) fn try_advance(&mut self) -> Result<&Token> {
        let span = self.cursor.span();
        self.cursor.advance()
            .ok_or_else(|| Error::new(span, ErrorKind::UnexpectedEof))
    }

    pub(super) fn try_peek(&self) -> Result<&Token> {
        let span = self.cursor.span();
        self.cursor.peek()
            .ok_or_else(|| Error::new(span, ErrorKind::UnexpectedEof))
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> Result<&Token> {
        let token = self.try_advance()?;

        if token.inner() == &kind {
            Ok(token)
        } else {
            Err(error::unexpected_token(token))
        }
    }

    pub(super) fn parse_block(&mut self) -> Result<Block> {
        self.expect(TokenKind::LBrace)?;
        let mut stmts = Block::new();

        loop {
            match self.cursor.peek().map(Token::inner) {
                Some(TokenKind::RBrace) => break,
                Some(_) => stmts.push(self.parse_stmt()?),
                None => return Err(Error::new(self.cursor.span(), ErrorKind::UnexpectedEof)),
            }
        }

        self.expect(TokenKind::RBrace)?;
        Ok(stmts)
    }

    pub(super) fn parse_ident(&mut self) -> Result<Ident> {
        let token = self.try_advance()?;

        if let TokenKind::Ident(s) = token.inner() {
            Ok(s.clone())
        } else {
            Err(error::unexpected_token(token))
        }
    }

    pub(super) fn parse_group<R, F>(
        &mut self,
        start: TokenKind,
        end: TokenKind,
        mut f: F,
    ) -> Result<Vec<R>>
    where
        F: FnMut(&mut Self) -> Result<R>,
    {
        // parses a group of tokens in the format <`start` <tokens> `end`>
        // `start` and `end` are both consumed, and `f` is called everytime
        // a `,` is seen. if `end` is seen before any `,`, this method
        // will return an empty `Vec`

        self.expect(start)?;
        let mut items = Vec::new();

        if self.cursor.consume_if(end.clone()) { return Ok(items); }

        loop {
            items.push(f(self)?);
            if !self.cursor.consume_if(TokenKind::Comma) {
                break;
            }
        }

        self.expect(end)?;
        Ok(items)
    }

    pub(super) fn with_span<F, R>(&mut self, f: F) -> Result<(Span, R)>
    where
        F: FnOnce(&mut Self) -> Result<R>,
    {
        let start = self.try_peek()?.span();
        let result = f(self);

        // using `.prev()` here if `f` did not consume any tokens, may result
        // in a span spanning before `start`, though it is not feasible to catch this
        let end = self.cursor.prev()
            .map(Token::span)
            .unwrap_or(Span::new(0, 0));
        result.map(|r| (start.merge(&end), r))
    }
}
