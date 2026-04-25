use std::fmt;
use crate::object::Builtin;

#[derive(Debug, Copy, Clone)]
pub struct Nil;

impl super::Type for Nil {
    fn type_name() -> &'static str {
        "nil"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for Nil {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nil")
    }
}

