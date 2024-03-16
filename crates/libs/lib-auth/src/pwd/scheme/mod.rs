//! A Scheme trait to enable our multi-scheme password hashing setup

use enum_dispatch::enum_dispatch;

mod error;
mod scheme_01;
mod scheme_02;

pub use self::error::{Error, Result};

use super::ContentToHash;

pub const DEFAULT_SCHEME: &str = "02";

/// An enum that marks whether a password uses the LATEST or an OUTDATED scheme.
#[derive(Debug)]
pub enum SchemeStatus {
    Ok,       // Password uses the latest scheme.
    Outdated, // Password uses an old scheme.
}

#[enum_dispatch]
pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String>;

    fn validate(&self, to_hash: &ContentToHash, raw_pwd_ref: &str) -> Result<()>;
}

#[enum_dispatch(Scheme)]
enum SchemeDispatch {
    Scheme01(scheme_01::Scheme01),
    Scheme02(scheme_02::Scheme02),
}

pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
    match scheme_name {
        "01" => Ok(SchemeDispatch::Scheme01(scheme_01::Scheme01)),
        "02" => Ok(SchemeDispatch::Scheme02(scheme_02::Scheme02)),
        _ => Err(Error::SchemeNotFound(scheme_name.into())),
    }
}
