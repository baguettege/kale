macro_rules! impl_display {
    ($target:ty => $method:ident) => {
        impl std::fmt::Display for $target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Printer::new(f).$method(self)
            }
        }
    };
}

mod types;
mod expr;
mod stmt;

use std::fmt::{Formatter, Result};

struct Printer<'a, 'b> {
    indent: usize,
    f: &'a mut Formatter<'b>,
}

impl<'a, 'b> Printer<'a, 'b> {
    fn new(f: &'a mut Formatter<'b>) -> Self {
        Self { indent: 0, f }
    }

    fn write_indent(&mut self) -> Result {
        write!(self.f, "{}", "    ".repeat(self.indent))
    }

    fn with_indent<F>(&mut self, f: F) -> Result
    where
        F: FnOnce(&mut Self) -> Result,
    {
        self.indent += 1;
        let result = f(self);
        self.indent -= 1;
        result
    }
}
