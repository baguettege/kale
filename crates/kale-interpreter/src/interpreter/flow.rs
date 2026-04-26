use kale_runtime::Error;
use kale_runtime::object::Object;

pub(super) enum Outcome {
    Error(Error),
    Return(Object),
}

pub(super) type Result<T> = std::result::Result<T, Outcome>;

impl From<Error> for Outcome {
    fn from(error: Error) -> Self {
        Self::Error(error)
    }
}
