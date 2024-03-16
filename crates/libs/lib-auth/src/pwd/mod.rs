use std::str::FromStr;
use uuid::Uuid;

mod error;
mod scheme;

pub use self::error::{Error, Result};
pub use self::scheme::SchemeStatus;

use self::scheme::Scheme;
use self::scheme::DEFAULT_SCHEME;

pub struct ContentToHash {
    pub content: String, // Clear content
    pub salt: Uuid,      // Clear salt.
}

/// Encrypt the password with the default scheme.
pub fn hash_pwd(to_hash: &ContentToHash) -> Result<String> {
    hash_for_scheme(DEFAULT_SCHEME, to_hash)
}

/// Validate a password provided by the user.
/// The user provided password is first encrypted with user's password salt.
/// Then the newly encrypted content is checked to verify that it's matching the encrypted
/// password in our database.
pub fn validate_pwd(to_hash: &ContentToHash, pwd_ref: &str) -> Result<SchemeStatus> {
    let PwdParts {
        scheme_name,
        hashed,
    } = pwd_ref.parse::<PwdParts>()?;
    validate_for_scheme(&scheme_name, to_hash, &hashed)?;

    if scheme_name == DEFAULT_SCHEME {
        Ok(SchemeStatus::Ok)
    } else {
        Ok(SchemeStatus::Outdated)
    }
}

fn hash_for_scheme(scheme_name: &str, to_hash: &ContentToHash) -> Result<String> {
    let scheme = scheme::get_scheme(scheme_name)?;
    let pwd_hashd = scheme.hash(to_hash)?;

    Ok(format!("#{scheme_name}#{pwd_hashd}"))
}

fn validate_for_scheme(
    scheme_name: &str,
    to_hash: &ContentToHash,
    raw_pwd_ref: &str,
) -> Result<()> {
    scheme::get_scheme(scheme_name)?.validate(to_hash, raw_pwd_ref)?;
    Ok(())
}

#[derive(Debug)]
struct PwdParts {
    /// The scheme only (e.g., "01")
    scheme_name: String,
    /// The hashed password
    hashed: String,
}

impl FromStr for PwdParts {
    type Err = Error;
    fn from_str(pwd_with_scheme: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let Some((scheme_name, hashed)) = pwd_with_scheme
            .strip_prefix("#")
            .ok_or(Error::InvalidParseFormat)?
            .split_once("#")
        else {
            return Err(Error::InvalidParseFormat);
        };

        Ok(Self {
            scheme_name: scheme_name.into(),
            hashed: hashed.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_multi_scheme_ok() -> Result<()> {
        // Setup & Fixtures
        let fx_salt = Uuid::parse_str("a6ed4554-1b19-4543-8c4d-aef508d01220")?;
        let fx_to_hash = ContentToHash {
            content: "hello world".into(),
            salt: fx_salt,
        };

        // Exec
        let pwd_hashd = hash_for_scheme("01", &fx_to_hash)?;
        let pwd_validate = validate_pwd(&fx_to_hash, &pwd_hashd)?;

        assert!(
            matches!(pwd_validate, SchemeStatus::Outdated),
            "status should be SchemeStatus::Outdated"
        );

        Ok(())
    }
}
