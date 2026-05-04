use crate::parser::Parser;
use kale_syntax::ast::{Closure, Expr, ExprKind, List, Literal};
use kale_syntax::token::TokenKind;
use crate::{error, Result};

impl Parser<'_> {
    pub(super) fn parse_primary(&mut self) -> Result<Expr> {
        let (span, kind) = self.with_span(|this| {
            let token = this.try_peek()?;
            match token.inner() {
                TokenKind::Nil |
                TokenKind::Num(_) |
                TokenKind::Bool(_) |
                TokenKind::Char(_) |
                TokenKind::Str(_) => this.parse_literal().map(Into::into),

                TokenKind::LParen => this.parse_grouped(),
                TokenKind::Ident(_) => this.parse_ident().map(Into::into),
                TokenKind::LBrack => this.parse_list().map(Into::into),
                TokenKind::Pipe => this.parse_closure().map(Into::into),
                _ => Err(error::unexpected_token(token)),
            }
        })?;
        
        Ok(Expr::new(span, kind))
    }

    fn parse_grouped(&mut self) -> Result<ExprKind> {
        self.expect(TokenKind::LParen)?;
        let expr = self.parse_expr()?;
        self.expect(TokenKind::RParen)?;
        
        // we throw away the internal span and return the kind
        // as the caller `parse_primary` is currently measuring the
        // span from `(` to `)`
        Ok(expr.into_inner())
    }

    fn parse_literal(&mut self) -> Result<Literal> {
        let token = self.try_advance()?;
        Ok(match token.inner() {
            TokenKind::Nil => Literal::Nil,
            TokenKind::Num(n) => Literal::Num(*n),
            TokenKind::Bool(b) => Literal::Bool(*b),
            TokenKind::Char(c) => Literal::Char(*c),
            TokenKind::Str(s) => Literal::Str(s.clone()),
            _ => return Err(error::unexpected_token(token)),
        })
    }

    fn parse_list(&mut self) -> Result<List> {
        let elements = self.parse_group(
            TokenKind::LBrack, TokenKind::RBrack, Self::parse_expr)?;
        Ok(List::new(elements))
    }

    fn parse_closure(&mut self) -> Result<Closure> {
        let params = self.parse_group(
            TokenKind::Pipe, TokenKind::Pipe, Self::parse_ident)?;
        let body = self.parse_block()?;
        Ok(Closure::new(params, body))
    }
}
