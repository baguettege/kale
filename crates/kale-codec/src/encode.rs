mod types;
mod primitives;
mod tag;
mod expr;
mod stmt;
mod collections;

pub(crate) struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    pub(crate) fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub(crate) fn put<T: AsRef<[u8]>>(&mut self, value: T) -> &mut Self {
        self.buf.extend_from_slice(value.as_ref());
        self
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub(crate) fn encode<T: Encode>(&mut self, value: &T) -> &mut Self {
        value.encode(self);
        self
    }
}

pub(crate) trait Encode {
    fn encode(&self, encoder: &mut Encoder);
}
