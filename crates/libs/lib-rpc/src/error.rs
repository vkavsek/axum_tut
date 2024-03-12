use lib_core::model;

use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Rpc
    MethodUnknown(String),
    MissingParams {
        rpc_method: String,
    },
    FailJsonParams {
        rpc_method: String,
    },

    // -- Modules
    #[from]
    Model(model::Error),
    // External modules
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::error::Error),
}

// Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
