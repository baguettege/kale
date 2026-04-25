mod bool;
mod builtin;
mod closure;
mod list;
mod module;
mod nil;
mod num;
mod str;
mod convert;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub use nil::Nil;
pub use num::Num;
pub use bool::Bool;
pub use str::Str;
pub use builtin::Builtin;
pub use closure::Closure;
pub use list::List;
pub use module::Module;

pub type Frozen<T> = Rc<T>;
pub type Mutable<T> = Rc<RefCell<T>>;

#[derive(Debug, Clone)]
pub enum Object {
    Nil(Nil),
    Num(Frozen<Num>),
    Bool(Frozen<Bool>),
    Str(Frozen<Str>),
    List(Mutable<List>),
    Closure(Frozen<Closure>),
    Builtin(&'static Builtin),
    Module(Mutable<Module>),
}

trait Type {
    fn type_name() -> &'static str;
    fn methods() -> &'static [Builtin];
}

impl Object {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Nil(_) => Nil::type_name(),
            Self::Num(_) => Num::type_name(),
            Self::Bool(_) => Bool::type_name(),
            Self::Str(_) => Str::type_name(),
            Self::List(_) => List::type_name(),
            Self::Closure(_) => Closure::type_name(),
            Self::Builtin(_) => Builtin::type_name(),
            Self::Module(_) => Module::type_name(),
        }
    }

    pub fn methods(&self) -> &'static [Builtin] {
        match self {
            Self::Nil(_) => Nil::methods(),
            Self::Num(_) => Num::methods(),
            Self::Bool(_) => Bool::methods(),
            Self::Str(_) => Str::methods(),
            Self::List(_) => List::methods(),
            Self::Closure(_) => Closure::methods(),
            Self::Builtin(_) => Builtin::methods(),
            Self::Module(_) => Module::methods(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil(_) => false,
            Self::Bool(b) => b.0,
            _ => true,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil(val) => write!(f, "{val}"),
            Self::Num(val) => write!(f, "{val}"),
            Self::Bool(val) => write!(f, "{val}"),
            Self::Str(val) => write!(f, "{val}"),
            Self::List(val) => write!(f, "{}", val.borrow()),
            Self::Closure(val) => write!(f, "{val}"),
            Self::Builtin(val) => write!(f, "{val}"),
            Self::Module(val) => write!(f, "{}", val.borrow()),
        }
    }
}
