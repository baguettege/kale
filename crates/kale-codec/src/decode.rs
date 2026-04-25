mod primitives;
mod tag;
mod collections;
mod types;
mod expr;
mod stmt;

use crate::{Error, Result};

pub(crate) struct Decoder<'a> {
    bytes: &'a [u8],
}

impl<'a> Decoder<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub(crate) fn take(&mut self, n: usize) -> Result<&[u8]> {
        match self.bytes.split_at_checked(n) {
            None => Err(Error::UnexpectedEof),
            Some((tail, head)) => {
                self.bytes = head;
                Ok(tail)
            }
        }
    }

    pub(crate) fn take_array<const N: usize>(&mut self) -> Result<&[u8; N]> {
        Ok(self.take(N)?.try_into().unwrap())
    }

    pub(crate) fn decode<T: Decode>(&mut self) -> Result<T> {
        T::decode(self)
    }
}

pub(crate) trait Decode: Sized {
    fn decode(decoder: &mut Decoder) -> Result<Self>;
}
