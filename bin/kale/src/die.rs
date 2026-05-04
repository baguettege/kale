use kale_api::report::{Diagnostic, Report};

pub(crate) fn or_die<T, E: Diagnostic>(source: &str, result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => die(source, &e),
    }
}

fn die<T: Diagnostic>(source: &str, diagnostic: &T) -> ! {
    let report = Report::new(source);
    eprintln!("{}", report.render(diagnostic));
    std::process::exit(1);
}
