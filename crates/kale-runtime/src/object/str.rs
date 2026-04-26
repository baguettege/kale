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
        use methods::*;
        use crate::builtin;

        const {
            &[
                builtin!("len", len),
                builtin!("to_lower", to_lower),
                builtin!("to_upper", to_upper),
                builtin!("at", at),
            ]
        }
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod methods {
    use crate::args::Args;
    use crate::object::{Frozen, Nil, Num, Object, Str};
    use crate::Result;

    pub(super) fn len(args: Args) -> Result<Object> {
        let s: Frozen<Str> = args.receiver()?.try_into()?;
        let len = s.0.chars().count();
        Ok(Num(len as f64).into())
    }

    pub(super) fn to_lower(args: Args) -> Result<Object> {
        let s: Frozen<Str> = args.receiver()?.try_into()?;
        let lower = s.0.to_lowercase();
        Ok(Str::new(lower).into())
    }

    pub(super) fn to_upper(args: Args) -> Result<Object> {
        let s: Frozen<Str> = args.receiver()?.try_into()?;
        let upper = s.0.to_uppercase();
        Ok(Str::new(upper).into())
    }

    pub(super) fn at(args: Args) -> Result<Object> {
        let s: Frozen<Str> = args.receiver()?.try_into()?;
        let index: Frozen<Num> = args.require(1)?.try_into()?;

        Ok(match s.0.chars().nth(index.0 as usize) {
            None => Nil.into(),
            Some(c) => Str::new(c.to_string()).into(),
        })
    }
}
