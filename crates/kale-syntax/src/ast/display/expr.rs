use crate::ast::display::Printer;
use crate::ast::{Binary, Call, Closure, Expr, Ident, Index, List, Member, Unary};
use std::fmt::Result;

impl Printer<'_, '_> {
    pub(crate) fn print_expr(&mut self, expr: &Expr) -> Result {
        match expr {
            Expr::Literal(lit) => write!(self.f, "{lit}"),
            Expr::Ident(ident) => write!(self.f, "{ident}"),
            Expr::Call(node) => self.print_call(node),
            Expr::Binary(node) => self.print_binary(node),
            Expr::Unary(node) => self.print_unary(node),
            Expr::List(node) => self.print_list(node),
            Expr::Closure(node) => self.print_closure(node),
            Expr::Member(node) => self.print_member(node),
            Expr::Index(node) => self.print_index(node),
        }
    }

    fn print_call(&mut self, node: &Call) -> Result {
        let args = node.args
            .iter()
            .map(|arg| format!("{arg}"))
            .collect::<Vec<_>>();
        write!(self.f, "{}({})", node.callee, args.join(", "))
    }

    fn print_binary(&mut self, node: &Binary) -> Result {
        write!(self.f, "{} {} {}", node.lhs, node.op, node.rhs)
    }

    fn print_unary(&mut self, node: &Unary) -> Result {
        write!(self.f, "{} {}", node.op, node.expr)
    }

    fn print_list(&mut self, node: &List) -> Result {
        let elements = node.elements
            .iter()
            .map(|elem| format!("{elem}"))
            .collect::<Vec<_>>();
        write!(self.f, "[{}]", elements.join(", "))
    }

    fn print_closure(&mut self, node: &Closure) -> Result {
        let params = node.params
            .iter()
            .map(Ident::as_str)
            .collect::<Vec<_>>();
        write!(self.f, "fn({}) ", params.join(", "))?;
        self.print_block(&node.body)
    }

    fn print_member(&mut self, node: &Member) -> Result {
        write!(self.f, "{}.{}", node.object, node.property)
    }

    fn print_index(&mut self, node: &Index) -> Result {
        write!(self.f, "{}[{}]", node.object, node.index)
    }
}

impl_display!(Expr => print_expr);
impl_display!(Call => print_call);
impl_display!(Binary => print_binary);
impl_display!(Unary => print_unary);
impl_display!(List => print_list);
impl_display!(Closure => print_closure);
impl_display!(Member => print_member);
impl_display!(Index => print_index);
