use std::fmt;
use crate::object::Object;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type {
    Nil,
    Num,
    Bool,
    Char,
    Str,
    List,
    Closure,
    BoundMethod,
    Module,
    StructDef,
    Struct,
    Native,
    NativeFn,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Num => write!(f, "num"),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),
            Self::Str => write!(f, "str"),
            Self::List => write!(f, "list"),
            Self::Closure => write!(f, "closure"),
            Self::BoundMethod => write!(f, "bound_method"),
            Self::Module => write!(f, "module"),
            Self::StructDef => write!(f, "struct_def"),
            Self::Struct => write!(f, "struct"),
            Self::Native => write!(f, "native"),
            Self::NativeFn => write!(f, "native_fn"),
        }
    }
}

impl From<Type> for String {
    fn from(ty: Type) -> Self {
        // implement so that we can use `impl Into<String>` for `Type`
        format!("{ty}")
    }
}

impl Object {
    pub fn ty(&self) -> Type {
        match self {
            Self::Nil => Type::Nil,
            Self::Num(_) => Type::Num,
            Self::Bool(_) => Type::Bool,
            Self::Char(_) => Type::Char,
            Self::Str(_) => Type::Str,
            Self::List(_) => Type::List,
            Self::Closure(_) => Type::Closure,
            Self::Module(_) => Type::Module,
            Self::Bound(_) => Type::BoundMethod,
            Self::StructDef(_) => Type::StructDef,
            Self::Struct(_) => Type::Struct,
            Self::Native(_) => Type::Native,
            Self::NativeFn(_) => Type::NativeFn,
        }
    }
}
