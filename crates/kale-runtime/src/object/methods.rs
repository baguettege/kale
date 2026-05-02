mod num;
mod str;
mod list;

use crate::object::{NativeMethod, Object};

impl Object {
    pub fn methods(&self) -> &'static [NativeMethod] {
        match self {
            Self::Num(_) => num::METHODS,
            Self::Str(_) => str::METHODS,
            Self::List(_) => list::METHODS,
            Self::Native(obj) => obj.borrow().methods(),
            _ => &[],
        }
    }
}
