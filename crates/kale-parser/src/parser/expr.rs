mod primary;
mod postfix;

use kale_syntax::ast::{BinOp, Binary, Expr, UnOp, Unary};
use kale_syntax::token::Token;
use crate::parser::expr::precedence::Precedence;
use crate::parser::Parser;
use crate::Result;

impl Parser<'_> {
    pub(super) fn parse_expr(&mut self) -> Result<Expr> {
        let lhs = self.parse_operand()?;
        self.parse_expr_inner(lhs, precedence::MIN)
    }

    fn parse_expr_inner(&mut self, mut lhs: Expr, min_precedence: Precedence) -> Result<Expr> {
        while let Some(op) = self.peek_binop() {
            let precedence = precedence::of(op);
            if precedence < min_precedence { break; }
            self.cursor.advance();

            let rhs = self.parse_operand()?;
            let rhs = self.parse_expr_inner(rhs, precedence + 1)?;

            lhs = Binary::new(lhs.into(), op, rhs.into()).into();
        }

        Ok(lhs)
    }

    fn parse_operand(&mut self) -> Result<Expr> {
        if let Some(op) = self.peek_unop() {
            self.cursor.advance();
            let expr = self.parse_operand()?;
            Ok(Unary::new(op, expr.into()).into())
        } else {
            let primary = self.parse_primary()?;
            self.parse_postfix(primary)
        }
    }
}

impl Parser<'_> {
    fn peek_binop(&self) -> Option<BinOp> {
        Some(match self.cursor.peek()? {
            Token::Star => BinOp::Mul,
            Token::Slash => BinOp::Div,
            Token::Percent => BinOp::Mod,

            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Sub,

            Token::Lt => BinOp::Lt,
            Token::Le => BinOp::Le,
            Token::Gt => BinOp::Gt,
            Token::Ge => BinOp::Ge,

            Token::EqEq => BinOp::Eq,
            Token::Ne => BinOp::Ne,
            Token::Is => BinOp::Is,

            Token::And => BinOp::And,

            Token::Or => BinOp::Or,

            _ => return None,
        })
    }

    fn peek_unop(&self) -> Option<UnOp> {
        Some(match self.cursor.peek()? {
            Token::Not => UnOp::Not,
            Token::Minus => UnOp::Neg,

            _ => return None,
        })
    }
}

mod precedence {
    use kale_syntax::ast::BinOp;

    pub(super) type Precedence = u32;

    pub(super) const MIN: Precedence = 0;

    pub(super) fn of(op: BinOp) -> Precedence {
        match op {
            BinOp::Mul | BinOp::Div | BinOp::Mod => 6,
            BinOp::Add | BinOp::Sub => 5,
            BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => 4,
            BinOp::Eq | BinOp::Ne | BinOp::Is => 3,
            BinOp::And => 2,
            BinOp::Or => 1,
        }
    }
}
