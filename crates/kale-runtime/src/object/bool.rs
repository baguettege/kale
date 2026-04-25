use std::fmt;
use crate::object::Builtin;

#[derive(Debug, Copy, Clone)]
pub struct Bool(pub bool);

impl super::Type for Bool {
    fn type_name() -> &'static str {
        "bool"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
