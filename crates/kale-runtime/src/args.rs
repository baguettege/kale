use crate::object::Object;
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Args<'a>(&'a [Object]);

impl<'a> Args<'a> {
    pub fn new(args: &'a [Object]) -> Self {
        Self(args)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<Object> {
        self.0.get(index).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = Object> {
        self.0.iter().cloned()
    }

    pub fn require(&self, index: usize) -> Result<Object> {
        self.get(index).ok_or(Error::MissingArg(index))
    }
}
