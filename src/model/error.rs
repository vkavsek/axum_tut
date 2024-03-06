use super::store;
use crate::crypt;

use derive_more::From;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, serde::Serialize, From)]
pub enum Error {
    ListLimitOverMax {
        max: i64,
        actual: i64,
    },
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    // Modules
    #[from]
    Crypt(crypt::Error),
    #[from]
    Store(store::Error),

    // External Errors
    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
    #[from]
    ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
