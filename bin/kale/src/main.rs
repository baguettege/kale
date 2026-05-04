use std::io;

mod run;
mod compile;
mod die;
mod exec;
mod show;

const USAGE: &str = "
usage: kale <cmd> <path>

commands:
 - compile <src>   compile source to .kast
 - run <path>      run a program
";

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("{USAGE}")]
    Usage,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Run(#[from] run::Error),
    #[error(transparent)]
    Show(#[from] show::Error),
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().ok_or(Error::Usage)?;
    let path = args.next().ok_or(Error::Usage)?;

    match cmd.as_str() {
        "compile" => Ok(compile::compile(&path)?),
        "run" => Ok(run::run(&path)?),
        "exec" => Ok(exec::exec(&path)?),
        "show" => Ok(show::show(&path)?),
        _ => Err(Error::Usage),
    }
}
