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
            Object::Nil(_) => Nil::type_name(),
            Object::Num(_) => Num::type_name(),
            Object::Bool(_) => Bool::type_name(),
            Object::Str(_) => Str::type_name(),
            Object::List(_) => List::type_name(),
            Object::Closure(_) => Closure::type_name(),
            Object::Builtin(_) => Builtin::type_name(),
            Object::Module(_) => Module::type_name(),
        }
    }

    pub fn methods(&self) -> &'static [Builtin] {
        match self {
            Object::Nil(_) => Nil::methods(),
            Object::Num(_) => Num::methods(),
            Object::Bool(_) => Bool::methods(),
            Object::Str(_) => Str::methods(),
            Object::List(_) => List::methods(),
            Object::Closure(_) => Closure::methods(),
            Object::Builtin(_) => Builtin::methods(),
            Object::Module(_) => Module::methods(),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Nil(val) => write!(f, "{val}"),
            Object::Num(val) => write!(f, "{val}"),
            Object::Bool(val) => write!(f, "{val}"),
            Object::Str(val) => write!(f, "{val}"),
            Object::List(val) => write!(f, "{}", val.borrow()),
            Object::Closure(val) => write!(f, "{val}"),
            Object::Builtin(val) => write!(f, "{val}"),
            Object::Module(val) => write!(f, "{}", val.borrow()),
        }
    }
}
