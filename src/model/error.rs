use serde_with::{serde_as, DisplayFromStr};

use crate::crypt;

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, serde::Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },
    // Crypt Errors
    Crypt(crypt::Error),
    // Model Errors
    Store(store::Error),
    // External Errors
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl From<crypt::Error> for Error {
    fn from(value: crypt::Error) -> Self {
        Self::Crypt(value)
    }
}

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Self::Store(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
