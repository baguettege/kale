use std::fmt;
use crate::object::{Builtin, Object};

#[derive(Debug, Clone)]
pub struct List(Vec<Object>);

impl List {
    pub fn new(elements: Vec<Object>) -> Self {
        Self(elements)
    }

    pub fn get(&self, index: usize) -> Option<Object> {
        self.0.get(index).cloned()
    }

    pub fn set(&mut self, index: usize, object: Object) -> Option<()> {
        if index < self.0.len() {
            self.0[index] = object;
            Some(())
        } else {
            None
        }
    }
}

impl super::Type for List {
    fn type_name() -> &'static str {
        "list"
    }

    fn methods() -> &'static [Builtin] {
        use methods::*;
        use crate::builtin;

        const {
            &[
                builtin!("len", len),
                builtin!("push", push),
                builtin!("pop", pop),
                builtin!("get", get),
            ]
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements = self.0
            .iter()
            .map(Object::to_string)
            .collect::<Vec<_>>();
        write!(f, "[{}]", elements.join(", "))
    }
}

mod methods {
    use crate::args::Args;
    use crate::object::{Frozen, List, Mutable, Nil, Num, Object};
    use crate::Result;

    pub(super) fn len(args: Args) -> Result<Object> {
        let list: Mutable<List> = args.receiver()?.try_into()?;
        let len = list.borrow().0.len();
        Ok(Num(len as f64).into())
    }

    pub(super) fn push(args: Args) -> Result<Object> {
        let list: Mutable<List> = args.receiver()?.try_into()?;
        let object = args.require(1)?;
        list.borrow_mut().0.push(object);
        Ok(Nil.into())
    }

    pub(super) fn pop(args: Args) -> Result<Object> {
        let list: Mutable<List> = args.receiver()?.try_into()?;
        let object = list
            .borrow_mut()
            .0
            .pop()
            .unwrap_or_else(|| Nil.into());
        Ok(object)
    }

    pub(super) fn get(args: Args) -> Result<Object> {
        let list: Mutable<List> = args.receiver()?.try_into()?;
        let index: Frozen<Num> = args.require(1)?.try_into()?;

        let object = list
            .borrow()
            .get(index.0 as usize)
            .unwrap_or_else(|| Nil.into());
        Ok(object)
    }
}
