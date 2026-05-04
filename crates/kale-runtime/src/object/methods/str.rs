use crate::args::Args;
use crate::ctx::Ctx;
use crate::object::{Immutable, NativeMethod, Object};
use crate::{native_method, Error, Result};

pub(super) const METHODS: &[NativeMethod] = &[
    native_method!("clone", clone),
    native_method!("len", len),
    native_method!("char_at", char_at),
    native_method!("chars", chars),
    native_method!("slice", slice),
    native_method!("to_lower", to_lower),
    native_method!("to_upper", to_upper),
    native_method!("trim", trim),
    native_method!("contains", contains),
    native_method!("split", split),
    native_method!("find", find),
    native_method!("starts_with", starts_with),
    native_method!("ends_with", ends_with),
    native_method!("replace", replace),
    native_method!("to_num", to_num),
];

pub fn clone(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    Ok(this.as_ref().clone().into())
}

pub fn len(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    this.chars().count().try_into()
}

pub fn char_at(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    let len = this.chars().count();

    let index: usize = args.require(0)?.try_into()?;

    let c = this.chars()
        .nth(index)
        .ok_or(Error::IndexOutOfBounds { index, len })?;
    Ok(c.into())
}

pub fn chars(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    let list = this.chars()
        .map(Object::from)
        .collect::<Vec<_>>();
    Ok(list.into())
}

pub fn slice(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    let len = this.chars().count();

    let start: usize = args.require(0)?.try_into()?;
    let end: usize = args.require(1)?.try_into()?;

    if start <= end && end <= len {
        let slice = this
            .chars()
            .skip(start)
            .take(end - start)
            .collect::<String>();
        Ok(slice.into())
    } else {
        let index = usize::max(start, end);
        Err(Error::IndexOutOfBounds { index, len })
    }
}

pub fn to_lower(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    Ok(this.to_lowercase().into())
}

pub fn to_upper(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    Ok(this.to_uppercase().into())
}

pub fn trim(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    Ok(this.trim().into())
}

pub fn contains(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    let substr: Immutable<String> = args.require(0)?.try_into()?;
    Ok(this.contains(substr.as_ref()).into())
}

pub fn split(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    let sep: Immutable<String> = args.require(0)?.try_into()?;

    let list = this
        .split(sep.as_ref())
        .map(str::to_string)
        .map(Object::from)
        .collect::<Vec<_>>();
    Ok(list.into())
}

pub fn find(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    let pat: Immutable<String> = args.require(0)?.try_into()?;

    match this.find(pat.as_ref()) {
        Some(i) => Ok(i.try_into()?),
        None => Ok(Object::Nil),
    }
}

pub fn starts_with(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    let prefix: Immutable<String> = args.require(0)?.try_into()?;
    Ok(this.starts_with(prefix.as_ref()).into())
}

pub fn ends_with(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    let suffix: Immutable<String> = args.require(0)?.try_into()?;
    Ok(this.ends_with(suffix.as_ref()).into())
}

pub fn replace(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;

    let from: Immutable<String> = args.require(0)?.try_into()?;
    let to: Immutable<String> = args.require(1)?.try_into()?;

    Ok(this.replace(from.as_ref(), to.as_ref()).into())
}

pub fn to_num(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: Immutable<String> = this.try_into()?;
    match this.parse::<f64>() {
        Ok(f) => Ok(f.into()),
        Err(_) => Ok(Object::Nil),
    }
}
