use kale_api::runtime::args::Args;
use kale_api::runtime::builtin::Lib;
use kale_api::runtime::object::{Frozen, Num, Object};
use kale_api::runtime::{builtin, lib, Result};

pub const MATH: &Lib = &lib!(
    "math",
    builtin!("floor", floor),
    builtin!("ceil", ceil),
    builtin!("sqrt", sqrt),
    builtin!("abs", abs),
);

fn floor(args: Args) -> Result<Object> {
    let n: Frozen<Num> = args.require(0)?.try_into()?;
    Ok(Num(n.0.floor()).into())
}

fn ceil(args: Args) -> Result<Object> {
    let n: Frozen<Num> = args.require(0)?.try_into()?;
    Ok(Num(n.0.ceil()).into())
}

fn sqrt(args: Args) -> Result<Object> {
    let n: Frozen<Num> = args.require(0)?.try_into()?;
    Ok(Num(n.0.sqrt()).into())
}

fn abs(args: Args) -> Result<Object> {
    let n: Frozen<Num> = args.require(0)?.try_into()?;
    Ok(Num(n.0.abs()).into())
}
