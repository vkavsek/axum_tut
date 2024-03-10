use uuid::Uuid;

mod error;
mod hmac_hasher;

use super::auth_config;

pub use self::error::{Error, Result};
use self::hmac_hasher::hmac_sha512_hash;

pub struct ContentToHash {
    pub content: String, // Clear content
    pub salt: Uuid,      // Clear salt.
}

/// Encrypt the password with the default scheme.
pub fn hash_pwd(to_hash: &ContentToHash) -> Result<String> {
    let key = &auth_config().PWD_KEY;

    let encrypted = hmac_sha512_hash(key, to_hash)?;

    Ok(format!("#01#{encrypted}"))
}

/// Validate a password provided by the user with the password stored in our database.
/// The user provided password is first encrypted with user's password salt.
/// Then the newly encrypted content is checked to verify that it's matching the encrypted
/// password in our database.
pub fn validate_pwd(to_hash: &ContentToHash, pwd_ref: &str) -> Result<()> {
    let pwd = hash_pwd(to_hash)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        Err(Error::PwdNotMatching)
    }
}
