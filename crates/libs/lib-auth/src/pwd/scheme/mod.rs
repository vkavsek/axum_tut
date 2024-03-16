//! A Scheme trait to enable our multi-scheme password hashing setup
mod error;
mod scheme_01;

pub use self::error::{Error, Result};

use super::ContentToHash;

pub const DEFAULT_SCHEME: &str = "01";

pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String>;

    fn validate(&self, to_hash: &ContentToHash, raw_pwd_ref: &str) -> Result<()>;
}

/// An enum that marks whether a password uses the LATEST or an OUTDATED scheme.
#[derive(Debug)]
pub enum SchemeStatus {
    Ok,       // Password uses the latest scheme.
    Outdated, // Password uses an old scheme.
}

pub fn get_scheme(scheme_name: &str) -> Result<Box<dyn Scheme>> {
    match scheme_name {
        "01" => Ok(Box::new(scheme_01::Scheme01)),
        _ => Err(Error::SchemeNotFound(scheme_name.into())),
    }
}
