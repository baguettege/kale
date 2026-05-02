use std::fmt::{Display, Formatter, Result};
use crate::ast::{BinOp, Ident, Literal, UnOp};
use crate::ast::pretty::Printer;

impl Printer<'_, '_> {
    pub(super) fn print_ident(&mut self, ident: &Ident) -> Result {
        write!(self.f, "{ident}")
    }

    pub(super) fn print_binop(&mut self, binop: BinOp) -> Result {
        write!(self.f, "{binop}")
    }

    pub(super) fn print_unop(&mut self, unop: UnOp) -> Result {
        write!(self.f, "{unop}")
    }

    pub(super) fn print_literal(&mut self, literal: &Literal) -> Result {
        write!(self.f, "{literal}")
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Is => write!(f, "is"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Not => write!(f, "not"),
            Self::Neg => write!(f, "-"),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Num(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Char(c) => write!(f, "'{c}'"),
            Self::Str(s) => write!(f, "\"{s}\""),
        }
    }
}
