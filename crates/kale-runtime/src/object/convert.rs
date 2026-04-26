use crate::object::{Bool, BoundMethod, Closure, Function, List, Module, Nil, Num, Str};
use crate::object::{Builtin, Frozen, Mutable, Object};
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! impl_from {
    ($target:ty, $variant:ident, $value:ident => $expr:expr) => {
        impl From<$target> for Object {
            fn from($value: $target) -> Self {
                Self::$variant($expr)
            }
        }
    };
}

macro_rules! impl_try_from {
    ($target:ty, $variant:ident, $inner:ty) => {
        impl TryFrom<Object> for $target {
            type Error = $crate::Error;

            fn try_from(object: Object) -> Result<Self, Self::Error> {
                match object {
                    Object::$variant(val) => Ok(val),
                    _ => Err($crate::Error::TypeError(
                        format!("expected {}, got {}",
                        <$inner as super::Type>::type_name(),
                        object.type_name())
                    )),
                }
            }
        }
    };
}

impl_from!(Nil, Nil, n => n);
impl_from!(Num, Num, n => Rc::new(n));
impl_from!(Bool, Bool, b => Rc::new(b));
impl_from!(Str, Str, s => Rc::new(s));
impl_from!(List, List, l => Rc::new(RefCell::new(l)));
impl_from!(Function, Function, f => Rc::new(f));
impl_from!(Closure, Closure, c => Rc::new(c));
impl_from!(&'static Builtin, Builtin, b => b);
impl_from!(Module, Module, m => Rc::new(RefCell::new(m)));
impl_from!(BoundMethod, Method, m => Rc::new(m));

impl_try_from!(Nil, Nil, Nil);
impl_try_from!(Frozen<Num>, Num, Num);
impl_try_from!(Frozen<Bool>, Bool, Bool);
impl_try_from!(Frozen<Str>, Str, Str);
impl_try_from!(Mutable<List>, List, List);
impl_try_from!(Frozen<Function>, Function, Function);
impl_try_from!(Frozen<Closure>, Closure, Closure);
impl_try_from!(&'static Builtin, Builtin, Builtin);
impl_try_from!(Mutable<Module>, Module, Module);
impl_try_from!(Frozen<BoundMethod>, Method, BoundMethod);
