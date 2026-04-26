use crate::object::Builtin;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Str(String);

impl Str {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl super::Type for Str {
    fn type_name() -> &'static str {
        "str"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
