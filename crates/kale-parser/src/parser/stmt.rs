use crate::parser::Parser;
use crate::{Error, Result};
use kale_syntax::ast::{Assign, FnDef, If, Module, Return, Stmt, While};
use kale_syntax::token::Token;

impl Parser<'_> {
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.cursor.peek().ok_or(Error::UnexpectedEof)? {
            Token::Module => self.parse_module().map(Into::into),
            Token::Fn if matches!(self.cursor.peek_ahead(1), Some(Token::Ident(_))) =>
                self.parse_fndef().map(Into::into),
            Token::If => self.parse_if().map(Into::into),
            Token::While => self.parse_while().map(Into::into),
            Token::Return => self.parse_return().map(Into::into),
            _ => self.parse_expr_or_assign(),
        }
    }

    fn parse_module(&mut self) -> Result<Module> {
        self.expect(Token::Module)?;
        let ident = self.parse_ident()?;
        let body = self.parse_block()?;
        Ok(Module::new(ident, body))
    }

    fn parse_fndef(&mut self) -> Result<FnDef> {
        self.expect(Token::Fn)?;
        let ident = self.parse_ident()?;

        let params = self.parse_group(
            Token::LParen, Token::RParen, Self::parse_ident)?;

        let body = self.parse_block()?;
        Ok(FnDef::new(ident, params, body))
    }

    fn parse_if(&mut self) -> Result<If> {
        self.expect(Token::If)?;

        let cond = self.parse_expr()?;
        let then_branch = self.parse_block()?;
        let else_branch = self.cursor
            .consume_if(Token::Else)
            .then(|| self.parse_block())
            .transpose()?;

        Ok(If::new(cond, then_branch, else_branch))
    }

    fn parse_while(&mut self) -> Result<While> {
        self.expect(Token::While)?;
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(While::new(cond, body))
    }

    fn parse_return(&mut self) -> Result<Return> {
        self.expect(Token::Return)?;
        let value = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(Return::new(value))
    }

    fn parse_expr_or_assign(&mut self) -> Result<Stmt> {
        let expr = self.parse_expr()?;

        if self.cursor.consume_if(Token::Assign) {
            let value = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            Ok(Assign::new(expr, value).into())
        } else {
            self.expect(Token::Semicolon)?;
            Ok(expr.into())
        }
    }
}
