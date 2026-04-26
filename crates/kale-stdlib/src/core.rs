use kale_api::runtime::args::Args;
use kale_api::runtime::{builtin, lib, Error, Result};
use kale_api::runtime::builtin::Lib;
use kale_api::runtime::object::{Bool, Num, Object, Str};

pub const CORE: &Lib = &lib!(
    "core",
    builtin!("type_of", type_of),
    builtin!("num", num),
    builtin!("bool", bool),
    builtin!("str", str),
);

fn type_of(args: Args) -> Result<Object> {
    let s = args.require(0)?.type_name();
    Ok(Str::new(s).into())
}

fn num(args: Args) -> Result<Object> {
    Ok(match args.require(0)? {
        Object::Num(n) => (*n).into(),
        Object::Str(s) => {
            s.as_str()
                .parse::<f64>()
                .map(|n| Num(n).into())
                .map_err(|e| Error::Runtime(
                    format!("could not cast \"{s}\" to num: {e}"))
                )?
        },
        object => return Err(Error::Runtime(
            format!("could not cast {object} to num"))
        ),
    })
}

fn bool(args: Args) -> Result<Object> {
    Ok(match args.require(0)? {
        Object::Nil(_) => Bool(false).into(),
        Object::Num(n) => Bool(n.0 != 0f64).into(),
        Object::Bool(b) => (*b).into(),
        Object::Str(s) => Bool(s.as_str() == "true").into(),
        object => return Err(Error::Runtime(
            format!("could not cast {object} to bool"))
        ),
    })
}

fn str(args: Args) -> Result<Object> {
    let object = args.require(0)?;
    Ok(Str::new(format!("{object}")).into())
}
