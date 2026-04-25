use std::fmt;
use crate::object::Builtin;

#[derive(Debug, Copy, Clone)]
pub struct Num(pub f64);

impl super::Type for Num {
    fn type_name() -> &'static str {
        "num"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

