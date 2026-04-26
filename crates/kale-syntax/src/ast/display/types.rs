use std::fmt::{Display, Formatter, Result};
use crate::ast::{BinOp, Literal, UnOp};

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
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
            Self::Num(val) => write!(f, "{val}"),
            Self::Bool(val) => write!(f, "{val}"),
            Self::Str(val) => write!(f, "\"{val}\""),
        }
    }
}
