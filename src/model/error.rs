use serde_with::{serde_as, DisplayFromStr};

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, serde::Serialize)]
pub enum Error {
    // Model Errors
    Store(store::Error),
    // External Errors
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
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
