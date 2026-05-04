use crate::args::Args;
use crate::ctx::Ctx;
use crate::{native_method, Error, Result};
use crate::object::{Mutable, NativeMethod, Object};

pub(super) const METHODS: &[NativeMethod] = &[
    native_method!("len", len),
    native_method!("is_empty", is_empty),
    native_method!("get", get),
    native_method!("set", set),
    native_method!("push", push),
    native_method!("pop", pop),
    native_method!("clone", clone),
    native_method!("to_str", to_str),
];

pub fn len(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Mutable<Vec<Object>> = this.try_into()?;
    this.borrow().len().try_into()
}

pub fn is_empty(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Mutable<Vec<Object>> = this.try_into()?;
    Ok(this.borrow().is_empty().into())
}

pub fn get(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Mutable<Vec<Object>> = this.try_into()?;
    let index: usize = args.require(0)?.try_into()?;

    let object = this.borrow()
        .get(index)
        .cloned()
        .unwrap_or(Object::Nil);
    Ok(object)
}

pub fn set(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Mutable<Vec<Object>> = this.try_into()?;
    let len = this.borrow().len();

    let index: usize = args.require(0)?.try_into()?;
    let value = args.require(1)?;

    this.borrow_mut()
        .get_mut(index)
        .map(|obj| *obj = value)
        .ok_or(Error::IndexOutOfBounds { index, len })?;
    Ok(Object::Nil)
}

pub fn push(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Mutable<Vec<Object>> = this.try_into()?;
    let value = args.require(0)?;

    this.borrow_mut().push(value);
    Ok(Object::Nil)
}

pub fn pop(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Mutable<Vec<Object>> = this.try_into()?;
    let object = this.borrow_mut()
        .pop()
        .unwrap_or(Object::Nil);
    Ok(object)
}

pub fn clone(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Mutable<Vec<Object>> = this.try_into()?;
    Ok(this.borrow().clone().into())
}

pub fn to_str(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    Ok(this.display().into())
}