use std::any;
use std::any::Any;
use std::fmt::{Debug, Display};
use crate::args::Args;
use crate::object::Object;
use crate::{Error, Result};
use crate::ctx::Ctx;

#[derive(Debug, Copy, Clone)]
pub struct NativeFn {
    pub func: fn(Ctx, Args) -> Result<Object>,
}

#[derive(Debug, Copy, Clone)]
pub struct NativeMethod {
    pub ident: &'static str,
    pub func: fn(Ctx, Object, Args) -> Result<Object>,
}

pub trait NativeObject: Any + Debug + Display {
    fn methods(&self) -> &'static [NativeMethod];
}

impl NativeFn {
    pub const fn new(func: fn(Ctx, Args) -> Result<Object>) -> Self {
        Self { func }
    }
}

impl NativeMethod {
    pub const fn new(
        ident: &'static str,
        func: fn(Ctx, Object, Args) -> Result<Object>,
    ) -> Self {
        Self { ident, func }
    }
}

impl dyn NativeObject {
    pub fn downcast_ref<T: NativeObject>(&self) -> Result<&T> {
        let got = any::type_name_of_val(self);
        (self as &dyn Any)
            .downcast_ref::<T>()
            .ok_or_else(|| Error::type_mismatch(any::type_name::<T>(), got))
    }

    pub fn downcast_mut<T: NativeObject>(&mut self) -> Result<&mut T> {
        let got = any::type_name_of_val(self);
        (self as &mut dyn Any)
            .downcast_mut::<T>()
            .ok_or_else(|| Error::type_mismatch(any::type_name::<T>(), got))
    }
}

mod macros {
    #[macro_export]
    macro_rules! native_fn {
        ($func:expr) => {
            $crate::object::NativeFn::new($func)
        };
    }

    #[macro_export]
    macro_rules! native_method {
        ($ident:literal, $func:expr) => {
            $crate::object::NativeMethod::new($ident, $func)
        };
    }
}
