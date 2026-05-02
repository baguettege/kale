use kale_api::interpreter::{Init, Setup};
use kale_api::runtime::args::Args;
use kale_api::runtime::ctx::Ctx;
use kale_api::runtime::object::Object;
use kale_api::runtime::{native_fn, Error, Result};

pub const INIT: Init = init;

fn init(setup: &mut Setup) {
    setup.module("io", |mut io| {
        io.define("print", native_fn!(print));
        io.define("println", native_fn!(println));
        io.define("readln", native_fn!(readln));
    });
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
