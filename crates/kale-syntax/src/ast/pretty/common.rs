use std::fmt::Result;
use crate::ast::pretty::Printer;

impl Printer<'_, '_> {
    pub(super) fn write_indent(&mut self) -> Result {
        write!(self.f, "{}", "    ".repeat(self.indent))
    }

    pub(super) fn with_indent<F>(&mut self, f: F) -> Result
    where
        F: FnOnce(&mut Self) -> Result,
    {
        self.indent += 1;
        let result = f(self);
        self.indent -= 1;
        result
    }

    pub(super) fn print_comma_separated<T, F>(&mut self, items: &[T], mut f: F) -> Result
    where
        F: FnMut(&mut Self, &T) -> Result,
    {
        for (i, item) in items.iter().enumerate() {
            if i > 0 { write!(self.f, ", ")?; }
            f(self, item)?;
        }

        Ok(())
    }
}
