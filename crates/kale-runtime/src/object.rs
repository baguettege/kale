mod method;
mod native;
mod ty;
mod closure;
mod module;
mod structs;
mod display;
mod methods;
mod convert;
mod semantics;

pub use method::{BoundMethod, Method};
pub use native::{NativeFn, NativeMethod, NativeObject};
pub use ty::Type;
pub use closure::Closure;
pub use module::Module;
pub use structs::{StructDef, Struct};

use std::cell::RefCell;
use std::rc::Rc;

pub type Immutable<T> = Rc<T>;
pub type Mutable<T> = Rc<RefCell<T>>;

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Num(f64),
    Bool(bool),
    Char(char),
    Str(Immutable<String>),
    List(Mutable<Vec<Object>>),
    Closure(Immutable<Closure>),
    Module(Mutable<Module>),
    Bound(Immutable<BoundMethod>),
    StructDef(Immutable<StructDef>),
    Struct(Mutable<Struct>),
    Native(Mutable<dyn NativeObject>),
    NativeFn(NativeFn),
}
