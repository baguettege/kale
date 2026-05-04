mod types;
mod expr;
mod stmt;
mod common;

use std::fmt::{Display, Formatter, Result};
use crate::ast::Program;

struct Printer<'a, 'b> {
    indent: usize,
    f: &'a mut Formatter<'b>,
}

impl<'a, 'b> Printer<'a, 'b> {
    fn new(f: &'a mut Formatter<'b>) -> Self {
        Self { indent: 0, f }
    }

    fn print_program(&mut self, program: &Program) -> Result {
        program.0.iter().try_for_each(|stmt| {
            self.print_stmt(stmt.inner())?;
            writeln!(self.f)
        })
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Printer::new(f).print_program(self)
    }
}
