use kale_syntax::ast::{Call, Expr, Index, Member};
use kale_syntax::token::Token;
use crate::parser::Parser;
use crate::Result;

impl Parser<'_> {
    pub(super) fn parse_postfix(&mut self, mut expr: Expr) -> Result<Expr> {
        loop {
             expr = match self.cursor.peek() {
                 Some(Token::LParen) => self.parse_call(expr)?.into(),
                 Some(Token::Dot) => self.parse_member(expr)?.into(),
                 Some(Token::LBrack) => self.parse_index(expr)?.into(),
                 _ => return Ok(expr),
            }
        }
    }

    fn parse_call(&mut self, callee: Expr) -> Result<Call> {
        let args = self.parse_group(
            Token::LParen, Token::RParen, Self::parse_expr)?;
        Ok(Call::new(callee.into(), args))
    }

    fn parse_member(&mut self, object: Expr) -> Result<Member> {
        self.expect(Token::Dot)?;
        let property = self.parse_ident()?;
        Ok(Member::new(object.into(), property))
    }

    fn parse_index(&mut self, object: Expr) -> Result<Index> {
        self.expect(Token::LBrack)?;
        let index = self.parse_expr()?;
        self.expect(Token::RBrack)?;
        Ok(Index::new(object.into(), index.into()))
    }
}
