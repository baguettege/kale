use kale_api::interpreter::{Loader, Registry};
use kale_api::runtime::args::Args;
use kale_api::runtime::ctx::Ctx;
use kale_api::runtime::object::Object;
use kale_api::runtime::{native_fn, Error, Result};

pub const LOADER: Loader = loader;

pub fn loader(registry: &mut Registry) {
    registry
        .define("type", native_fn!(type_of))
        .define("print", native_fn!(print))
        .define("println", native_fn!(println))
        .define("readln", native_fn!(readln))
        .define("assert", native_fn!(assert))
        .define("or_else", native_fn!(or_else));
}

pub fn type_of(_ctx: Ctx, args: Args) -> Result<Object> {
    let object = args.require(0)?;
    Ok(object.ty().to_string().into())
}

pub fn print(_ctx: Ctx, args: Args) -> Result<Object> {
    let output = args
        .iter()
        .map(|arg| arg.display())
        .collect::<Vec<_>>()
        .join(" ");

    print!("{output}");
    Ok(Object::Nil)
}

pub fn println(ctx: Ctx, args: Args) -> Result<Object> {
    print(ctx, args)?;
    println!();
    Ok(Object::Nil)
}

pub fn readln(ctx: Ctx, args: Args) -> Result<Object> {
    print(ctx, args)?;

    // print! doesn't always print to the terminal
    // immediately w/o a newline
    use std::io::Write;
    std::io::stdout()
        .flush()
        .map_err(|e| Error::raise(format!("failed to flush stdout: {e}")))?;

    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .map_err(|e| Error::raise(format!("failed to read stdin: {e}")))?;

    Ok(buf.trim_end_matches(['\n', '\r']).into())
}

pub fn assert(_ctx: Ctx, args: Args) -> Result<Object> {
    let condition: bool = args.require(0)?.try_into()?;

    if !condition {
        if let Some(msg) = args.get(1) {
            Err(Error::raise(format!("assertion failed: {}", msg.display())))
        } else {
            Err(Error::raise("assertion failed"))
        }
    } else {
        Ok(Object::Nil)
    }
}

pub fn or_else(_ctx: Ctx, args: Args) -> Result<Object> {
    let value = args.require(0)?;
    let fallback = args.require(1)?;

    Ok(match value {
        Object::Nil => fallback,
        _ => value,
    })
}
