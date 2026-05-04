use crate::parser::Parser;
use crate::Result;
use kale_syntax::ast::{Assign, FnDef, If, Let, Module, Raise, Return, Stmt, StmtKind, Struct, While};
use kale_syntax::token::{Token, TokenKind};

impl Parser<'_> {
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        let (span, kind) = self.with_span(Self::parse_stmt_kind)?;
        Ok(Stmt::new(span, kind))
    }

    fn parse_stmt_kind(&mut self) -> Result<StmtKind> {
        match self.try_peek()?.inner() {
            TokenKind::Module => self.parse_module().map(StmtKind::from),
            TokenKind::Struct => self.parse_struct().map(StmtKind::from),
            TokenKind::Fn => self.parse_fndef().map(StmtKind::from),
            TokenKind::Let => self.parse_let().map(StmtKind::from),
            TokenKind::If => self.parse_if().map(StmtKind::from),
            TokenKind::While => self.parse_while().map(StmtKind::from),
            TokenKind::Return => self.parse_return().map(StmtKind::from),
            TokenKind::Raise => self.parse_raise().map(StmtKind::from),
            _ => self.parse_expr_or_assign(),
        }
    }

    fn parse_module(&mut self) -> Result<Module> {
        self.expect(TokenKind::Module)?;
        let ident = self.parse_ident()?;
        let body = self.parse_block()?;
        Ok(Module::new(ident, body))
    }

    fn parse_struct(&mut self) -> Result<Struct> {
        self.expect(TokenKind::Struct)?;
        let ident = self.parse_ident()?;

        let fields = self.parse_group(
            TokenKind::LParen, TokenKind::RParen, Self::parse_ident)?;

        let mut methods = Vec::new();
        self.expect(TokenKind::LBrace)?;

        while !matches!(self.cursor.peek().map(Token::inner), Some(TokenKind::RBrace)) {
            methods.push(self.parse_fndef()?);
        }

        self.expect(TokenKind::RBrace)?;
        Ok(Struct::new(ident, fields, methods))
    }

    fn parse_fndef(&mut self) -> Result<FnDef> {
        self.expect(TokenKind::Fn)?;
        let ident = self.parse_ident()?;

        let params = self.parse_group(
            TokenKind::LParen, TokenKind::RParen, Self::parse_ident)?;

        let body = self.parse_block()?;
        Ok(FnDef::new(ident, params, body))
    }

    fn parse_let(&mut self) -> Result<Let> {
        self.expect(TokenKind::Let)?;
        let ident = self.parse_ident()?;
        self.expect(TokenKind::Assign)?;
        let init = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Let::new(ident, init))
    }

    fn parse_if(&mut self) -> Result<If> {
        self.expect(TokenKind::If)?;

        let cond = self.parse_expr()?;
        let then_branch = self.parse_block()?;
        let else_branch = self.cursor
            .consume_if(TokenKind::Else)
            .then(|| self.parse_block())
            .transpose()?;

        Ok(If::new(cond, then_branch, else_branch))
    }

    fn parse_while(&mut self) -> Result<While> {
        self.expect(TokenKind::While)?;
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(While::new(cond, body))
    }

    fn parse_return(&mut self) -> Result<Return> {
        self.expect(TokenKind::Return)?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Return::new(value))
    }

    fn parse_raise(&mut self) -> Result<Raise> {
        self.expect(TokenKind::Raise)?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Raise::new(value))
    }

    fn parse_expr_or_assign(&mut self) -> Result<StmtKind> {
        let expr = self.parse_expr()?;

        if self.cursor.consume_if(TokenKind::Assign) {
            let value = self.parse_expr()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Assign::new(expr, value).into())
        } else {
            self.expect(TokenKind::Semicolon)?;
            Ok(expr.into())
        }
    }
}
