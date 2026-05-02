use std::fmt::{Display, Formatter, Result};
use kale_syntax::ast::Ident;
use crate::object::{BoundMethod, NativeFn, Closure, Module, Mutable, Object, Struct, StructDef};

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        fn format_list(list: &Mutable<Vec<Object>>) -> Vec<String> {
            list.borrow()
                .iter()
                .map(Object::to_string)
                .collect()
        }

        match self {
            Self::Nil => write!(f, "nil"),
            Self::Num(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Char(c) => write!(f, "'{c}'"),
            Self::Str(s) => write!(f, "\"{s}\""),
            Self::List(list) => write!(f, "[{}]", format_list(list).join(", ")),
            Self::Closure(closure) => write!(f, "{closure}"),
            Self::Module(module) => write!(f, "{}", module.borrow()),
            Self::Bound(bound) => write!(f, "{bound}"),
            Self::StructDef(def) => write!(f, "{def}"),
            Self::Struct(instance) => write!(f, "{}", instance.borrow()),
            Self::Native(object) => write!(f, "{}", object.borrow()),
            Self::NativeFn(builtin) => write!(f, "{builtin}"),
        }
    }
}

impl Display for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "fn({})", self.params().join(", "))
    }
}

impl Display for NativeFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<native_fn>")
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<module>")
    }
}

impl Display for BoundMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<bound_method>")
    }
}

impl Display for StructDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let fields = self.fields
            .iter()
            .map(Ident::as_str)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "struct({fields})")
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let fields = self.fields
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "struct({fields})")
    }
}
