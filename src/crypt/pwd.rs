use crate::config;

use super::{encrypt_into_b64u, EncryptContent, Error, Result};

/// Encrypt the password with the default scheme.
pub fn encrypt_pwd(to_encrypt: &EncryptContent) -> Result<String> {
    let key = &config().PWD_KEY;

    let encrypted = encrypt_into_b64u(key, to_encrypt)?;

    Ok(format!("#01#{encrypted}"))
}

/// Validate a password provided by the user with the password stored in our database.
/// The user provided password is first encrypted with user's password salt.
/// Then the newly encrypted content is checked to verify that it's matching the encrypted
/// password in our database.
pub fn validate_pwd(to_encrypt: &EncryptContent, pwd_ref: &str) -> Result<()> {
    let pwd = encrypt_pwd(to_encrypt)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        Err(Error::PwdNotMatching)
    }
}
