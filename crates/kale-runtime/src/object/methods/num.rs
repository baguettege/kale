use crate::args::Args;
use crate::ctx::Ctx;
use crate::object::{NativeMethod, Object};
use crate::{native_method, Result};

pub(super) const METHODS: &[NativeMethod] = &[
    native_method!("to_str", to_str),
    native_method!("min", min),
    native_method!("max", max),
    native_method!("abs", abs),
    native_method!("floor", floor),
    native_method!("ceil", ceil),
    native_method!("round", round),
];

pub fn to_str(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    Ok(this.display().into())
}

pub fn min(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: f64 = this.try_into()?;
    let other: f64 = args.require(0)?.try_into()?;
    Ok(f64::min(this, other).into())
}

pub fn max(_ctx: Ctx, this: Object, args: Args) -> Result<Object> {
    let this: f64 = this.try_into()?;
    let other: f64 = args.require(0)?.try_into()?;
    Ok(f64::max(this, other).into())
}

pub fn abs(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: f64 = this.try_into()?;
    Ok(this.abs().into())
}

pub fn floor(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: f64 = this.try_into()?;
    Ok(this.floor().into())
}

pub fn ceil(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: f64 = this.try_into()?;
    Ok(this.ceil().into())
}

pub fn round(_ctx: Ctx, this: Object, _args: Args) -> Result<Object> {
    let this: f64 = this.try_into()?;
    Ok(this.round().into())
}
