use kale_syntax::ast::{Call, Expr, Member};
use kale_syntax::token::{Token, TokenKind};
use crate::parser::Parser;
use crate::Result;

impl Parser<'_> {
    pub(super) fn parse_postfix(&mut self, mut expr: Expr) -> Result<Expr> {
        loop {
            let start = expr.span();

            let (end, kind) = match self.cursor.peek().map(Token::inner) {
                // `self.with_span` is called per arm here as the closure cannot `break`
                // the outer loop, and the default case `_ => break` wouldn't
                // satisfy the closure's return type
                Some(TokenKind::LParen) => self.with_span(|this| {
                    Ok(this.parse_call(expr)?.into())
                })?,
                Some(TokenKind::Dot) => self.with_span(|this| {
                    Ok(this.parse_member(expr)?.into())
                })?,
                _ => break,
            };

            expr = Expr::new(start.merge(&end), kind);
        }

        Ok(expr)
    }

    fn parse_call(&mut self, callee: Expr) -> Result<Call> {
        let args = self.parse_group(
            TokenKind::LParen, TokenKind::RParen, Self::parse_expr)?;
        Ok(Call::new(callee.into(), args))
    }

    fn parse_member(&mut self, object: Expr) -> Result<Member> {
        self.expect(TokenKind::Dot)?;
        let property = self.parse_ident()?;
        Ok(Member::new(object.into(), property))
    }
}
