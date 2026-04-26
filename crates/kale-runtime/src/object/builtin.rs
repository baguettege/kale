use crate::object::Object;
use crate::Result;
use std::fmt;
use crate::args::Args;

pub type BuiltinFn = fn(Args) -> Result<Object>;

#[derive(Debug, Clone)]
pub struct Builtin {
    pub ident: &'static str,
    pub func: BuiltinFn,
}

impl Builtin {
    pub const fn new(ident: &'static str, func: BuiltinFn) -> Self {
        Self { ident, func }
    }
}

impl super::Type for Builtin {
    fn type_name() -> &'static str {
        "builtin"
    }

    fn methods() -> &'static [Builtin] {
        &[]
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<builtin:{}>", self.ident)
    }
}


#[macro_export]
macro_rules! builtin {
    ($ident:literal, $func:expr) => {
        $crate::object::Builtin::new($ident, $func)
    };
}
