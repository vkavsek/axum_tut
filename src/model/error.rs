use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, serde::Serialize)]
pub enum Error {
    // Model Errors
    Store(store::Error),
}

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Self::Store(value)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
