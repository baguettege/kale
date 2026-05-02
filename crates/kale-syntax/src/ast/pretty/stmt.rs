use crate::ast::pretty::Printer;
use crate::ast::{Assign, Block, Expr, FnDef, If, Let, Module, Raise, Return, Stmt, Struct, While};
use std::fmt::Result;

impl Printer<'_, '_> {
    pub(super) fn print_block(&mut self, block: &Block) -> Result {
        writeln!(self.f, "{{")?;

        self.with_indent(|this| {
            block.iter().try_for_each(|stmt| {
                this.print_stmt(stmt)?;
                writeln!(this.f)
            })
        })?;

        self.write_indent()?;
        write!(self.f, "}}")
    }

    pub(super) fn print_stmt(&mut self, stmt: &Stmt) -> Result {
        self.write_indent()?;

        match stmt {
            Stmt::Expr(expr) => self.print_expr_stmt(expr),
            Stmt::Module(node) => self.print_module(node),
            Stmt::Struct(node) => self.print_struct(node),
            Stmt::FnDef(node) => self.print_fndef(node),
            Stmt::Let(node) => self.print_let(node),
            Stmt::Assign(node) => self.print_assign(node),
            Stmt::If(node) => self.print_if(node),
            Stmt::While(node) => self.print_while(node),
            Stmt::Return(node) => self.print_return(node),
            Stmt::Raise(node) => self.print_raise(node),
        }
    }

    fn print_expr_stmt(&mut self, expr: &Expr) -> Result {
        self.print_expr(expr)?;
        write!(self.f, ";")
    }

    fn print_module(&mut self, node: &Module) -> Result {
        write!(self.f, "module {} ", node.ident)?;
        self.print_block(&node.body)
    }

    fn print_struct(&mut self, node: &Struct) -> Result {
        write!(self.f, "struct {}(", node.ident)?;
        self.print_comma_separated(&node.fields, Self::print_ident)?;
        write!(self.f, ") {{")?;

        self.with_indent(|this| {
            node.methods.iter().try_for_each(|method| {
                this.print_fndef(method)?;
                writeln!(this.f)
            })
        })?;

        write!(self.f, "}}")
    }

    fn print_fndef(&mut self, node: &FnDef) -> Result {
        write!(self.f, "fn {}(", node.ident)?;
        self.print_comma_separated(&node.params, Self::print_ident)?;
        write!(self.f, ") ")?;
        self.print_block(&node.body)
    }

    fn print_let(&mut self, node: &Let) -> Result {
        write!(self.f, "let {} = ", node.ident)?;
        self.print_expr(&node.init)?;
        write!(self.f, ";")
    }

    fn print_assign(&mut self, node: &Assign) -> Result {
        self.print_expr(&node.target)?;
        write!(self.f, " = ")?;
        self.print_expr(&node.value)?;
        write!(self.f, ";")
    }

    fn print_if(&mut self, node: &If) -> Result {
        write!(self.f, "if ")?;
        self.print_expr(&node.cond)?;
        write!(self.f, " ")?;
        self.print_block(&node.then_branch)?;

        if let Some(else_branch) = &node.else_branch {
            write!(self.f, " else ")?;
            self.print_block(else_branch)?;
        }

        Ok(())
    }

    fn print_while(&mut self, node: &While) -> Result {
        write!(self.f, "while ")?;
        self.print_expr(&node.cond)?;
        write!(self.f, " ")?;
        self.print_block(&node.body)
    }

    fn print_return(&mut self, node: &Return) -> Result {
        write!(self.f, "return ")?;
        self.print_expr(&node.value)?;
        write!(self.f, ";")
    }

    fn print_raise(&mut self, node: &Raise) -> Result {
        write!(self.f, "raise ")?;
        self.print_expr(&node.value)?;
        write!(self.f, ";")
    }
}
