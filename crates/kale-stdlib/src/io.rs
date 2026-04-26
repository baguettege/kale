use kale_api::runtime::builtin::Lib;
use kale_api::runtime::{builtin, lib, Result};
use kale_api::runtime::args::Args;
use kale_api::runtime::object::{Nil, Object};

pub const IO: &Lib = &lib!(
    "io",
    builtin!("print", print),
    builtin!("println", println),
);

fn print(args: Args) -> Result<Object> {
    args.iter().for_each(|arg| print!("{arg}"));
    Ok(Nil.into())
}

fn println(args: Args) -> Result<Object> {
    args.iter().for_each(|arg| print!("{arg}"));
    println!();
    Ok(Nil.into())
}
