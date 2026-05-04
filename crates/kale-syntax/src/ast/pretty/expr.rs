use crate::ast::pretty::Printer;
use crate::ast::{Binary, Call, Closure, Expr, ExprKind, List, Member, Unary};
use std::fmt::Result;

impl Printer<'_, '_> {
    pub(crate) fn print_expr(&mut self, expr: &Expr) -> Result {
        match expr.inner() {
            ExprKind::Literal(lit) => self.print_literal(lit),
            ExprKind::Ident(ident) => self.print_ident(ident),
            ExprKind::Call(node) => self.print_call(node),
            ExprKind::Binary(node) => self.print_binary(node),
            ExprKind::Unary(node) => self.print_unary(node),
            ExprKind::List(node) => self.print_list(node),
            ExprKind::Closure(node) => self.print_closure(node),
            ExprKind::Member(node) => self.print_member(node),
        }
    }

    fn print_call(&mut self, node: &Call) -> Result {
        self.print_expr(&node.callee)?;
        write!(self.f, "(")?;
        self.print_comma_separated(&node.args, Self::print_expr)?;
        write!(self.f, ")")
    }

    fn print_binary(&mut self, node: &Binary) -> Result {
        self.print_expr(&node.lhs)?;
        write!(self.f, " ")?;
        self.print_binop(node.op)?;
        write!(self.f, " ")?;
        self.print_expr(&node.rhs)
    }

    fn print_unary(&mut self, node: &Unary) -> Result {
        self.print_unop(node.op)?;
        write!(self.f, " ")?;
        self.print_expr(&node.expr)
    }

    fn print_list(&mut self, node: &List) -> Result {
        write!(self.f, "[")?;
        self.print_comma_separated(&node.elements, Self::print_expr)?;
        write!(self.f, "]")
    }

    fn print_closure(&mut self, node: &Closure) -> Result {
        write!(self.f, "fn(")?;
        self.print_comma_separated(&node.params, Self::print_ident)?;
        write!(self.f, ") ")?;
        self.print_block(&node.body)
    }

    fn print_member(&mut self, node: &Member) -> Result {
        self.print_expr(&node.object)?;
        write!(self.f, ".")?;
        self.print_ident(&node.property)
    }
}
