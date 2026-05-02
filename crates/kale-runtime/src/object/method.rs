use crate::object::{Closure, Immutable, NativeMethod, Object};

#[derive(Debug)]
pub struct BoundMethod {
    pub(super) receiver: Object,
    pub(super) method: Method,
}

#[derive(Debug, Clone)]
pub enum Method {
    Native(&'static NativeMethod),
    Closure(Immutable<Closure>),
}

impl BoundMethod {
    pub fn new(receiver: impl Into<Object>, method: Method) -> Self {
        let receiver = receiver.into();
        Self { receiver, method }
    }

    pub fn receiver(&self) -> Object {
        self.receiver.clone()
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }
}
