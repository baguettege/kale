use std::rc::Rc;
use crate::object::Object;

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(b) => *b,
            _ => true,
        }
    }

    pub fn is(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Num(a), Self::Num(b)) => a == b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Char(a), Self::Char(b)) => a == b,

            (Self::Str(a), Self::Str(b)) => Rc::ptr_eq(a, b),
            (Self::List(a), Self::List(b)) => Rc::ptr_eq(a, b),
            (Self::Closure(a), Self::Closure(b)) => Rc::ptr_eq(a, b),
            (Self::Module(a), Self::Module(b)) => Rc::ptr_eq(a, b),
            (Self::Bound(a), Self::Bound(b)) => Rc::ptr_eq(a, b),
            (Self::StructDef(a), Self::StructDef(b)) => Rc::ptr_eq(a, b),
            (Self::Struct(a), Self::Struct(b)) => Rc::ptr_eq(a, b),
            (Self::Native(a), Self::Native(b)) => Rc::ptr_eq(a, b),

            (Self::NativeFn(a), Self::NativeFn(b)) => std::ptr::eq(a, b),

            _ => false,
        }
    }

    pub fn display(&self) -> String {
        match self {
            // unquote the string for user-facing ouput
            Self::Str(s) => s.to_string(),
            _ => format!("{self}"),
        }
    }
}
