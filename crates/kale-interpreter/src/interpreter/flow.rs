use kale_runtime::Error;
use kale_runtime::object::Object;

#[derive(Debug)]
pub(super) enum Signal {
    Error(Error),
    Return(Object),
}

pub(super) type Result<T> = std::result::Result<T, Signal>;

impl From<Error> for Signal {
    fn from(error: Error) -> Self {
        Self::Error(error)
    }
}
