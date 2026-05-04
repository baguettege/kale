use crate::args::Args;
use crate::ctx::Ctx;
use crate::object::{NativeMethod, Object};
use crate::{native_method, Result};

pub(super) const METHODS: &[NativeMethod] = &[
    native_method!("to_str", to_str),
    native_method!("is_digit", is_digit),
    native_method!("is_whitespace", is_whitespace),
    native_method!("is_alphabetic", is_alphabetic),
    native_method!("is_alphanumeric", is_alphanumeric),
];

pub fn to_str(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: char = this.try_into()?;
    Ok(this.to_string().into())
}

pub fn is_digit(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: char = this.try_into()?;
    Ok(this.is_digit(10).into())
}

pub fn is_whitespace(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: char = this.try_into()?;
    Ok(this.is_whitespace().into())
}

pub fn is_alphabetic(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: char = this.try_into()?;
    Ok(this.is_alphabetic().into())
}


pub fn is_alphanumeric(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: char = this.try_into()?;
    Ok(this.is_alphanumeric().into())
}