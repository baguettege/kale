use crate::parser::Parser;
use crate::{Error, Result};
use kale_syntax::ast::{Closure, Expr, List, Literal};
use kale_syntax::token::Token;

impl Parser<'_> {
    pub(super) fn parse_primary(&mut self) -> Result<Expr> {
        match self.cursor.peek().ok_or(Error::UnexpectedEof)? {
            Token::LParen => self.parse_grouped(),
            Token::Nil | Token::Num(_) | Token::Bool(_) | Token::Char(_) | Token::Str(_) =>
                self.parse_literal().map(Into::into),
            Token::Ident(_) => self.parse_ident().map(Into::into),
            Token::LBrack => self.parse_list().map(Into::into),
            Token::Fn if matches!(self.cursor.peek_ahead(1), Some(Token::LParen)) =>
                self.parse_closure().map(Into::into),
            tok => Err(Error::UnexpectedToken(tok.to_string())),
        }
    }

    fn parse_grouped(&mut self) -> Result<Expr> {
        self.expect(Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect(Token::RParen)?;
        Ok(expr)
    }

    fn parse_literal(&mut self) -> Result<Literal> {
        Ok(match self.cursor.advance().ok_or(Error::UnexpectedEof)? {
            Token::Nil => Literal::Nil,
            Token::Num(n) => Literal::Num(*n),
            Token::Bool(b) => Literal::Bool(*b),
            Token::Char(c) => Literal::Char(*c),
            Token::Str(s) => Literal::Str(s.clone()),
            tok => return Err(Error::UnexpectedToken(tok.to_string())),
        })
    }

    fn parse_list(&mut self) -> Result<List> {
        let elements = self.parse_group(
            Token::LBrack, Token::RBrack, Self::parse_expr)?;
        Ok(List::new(elements))
    }

    fn parse_closure(&mut self) -> Result<Closure> {
        self.expect(Token::Fn)?;
        let params = self.parse_group(
            Token::LParen, Token::RParen, Self::parse_ident)?;
        let body = self.parse_block()?;
        Ok(Closure::new(params, body))
    }
}
