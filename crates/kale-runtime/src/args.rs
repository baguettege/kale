use crate::object::Object;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Args<'a>(pub &'a [Object]);

impl<'a> Args<'a> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<Object> {
        self.0.get(index).cloned()
    }

    pub fn require(&self, index: usize) -> Result<Object> {
        match self.get(index) {
            None => Err(Error::MissingArg(index)),
            Some(obj) => Ok(obj.clone()),
        }
    }

    pub fn receiver(&self) -> Result<Object> {
        self.require(0)
    }

    pub fn iter(&self) -> impl Iterator<Item = Object> {
        self.0.iter().cloned()
    }
}
