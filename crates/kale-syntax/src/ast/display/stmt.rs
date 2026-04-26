use crate::ast::display::Printer;
use crate::ast::{Assign, Block, Expr, FnDef, Ident, If, Module, Return, Stmt, While};
use std::fmt::Result;

impl Printer<'_, '_> {
    pub(crate) fn print_block(&mut self, block: &Block) -> Result {
        writeln!(self.f, "{{")?;

        self.with_indent(|this| {
            for stmt in &block.0 {
                this.print_stmt(stmt)?;
                writeln!(this.f)?;
            }
            Ok(())
        })?;

        self.write_indent()?;
        write!(self.f, "}}")
    }

    fn print_stmt(&mut self, stmt: &Stmt) -> Result {
        self.write_indent()?;

        match stmt {
            Stmt::Expr(expr) => self.print_expr_stmt(expr),
            Stmt::Module(node) => self.print_module(node),
            Stmt::FnDef(node) => self.print_fndef(node),
            Stmt::Assign(node) => self.print_assign(node),
            Stmt::If(node) => self.print_if(node),
            Stmt::While(node) => self.print_while(node),
            Stmt::Return(node) => self.print_return(node),
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

    fn print_fndef(&mut self, node: &FnDef) -> Result {
        let params = node.params
            .iter()
            .map(Ident::as_str)
            .collect::<Vec<_>>()
            .join(", ");

        write!(self.f, "fn {}({params}) ", node.ident)?;
        self.print_block(&node.body)
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
}

impl_display!(Block => print_block);
impl_display!(Stmt => print_stmt);
impl_display!(Module => print_module);
impl_display!(FnDef => print_fndef);
impl_display!(Assign => print_assign);
impl_display!(If => print_if);
impl_display!(While => print_while);
impl_display!(Return => print_return);
