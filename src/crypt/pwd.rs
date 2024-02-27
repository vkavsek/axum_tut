use crate::config;

use super::{encrypt_into_b64u, EncryptContent, Error, Result};

/// Encrypt the password with the default scheme.
pub fn encrypt_pwd(to_encrypt: &EncryptContent) -> Result<String> {
    let key = &config().PWD_KEY;

    let encrypted = encrypt_into_b64u(key, to_encrypt)?;

    Ok(format!("#01#{encrypted}"))
}

/// Validate if an EncryptContent matches.
pub fn validate_pwd(encrypt_content: &EncryptContent, pwd_ref: &str) -> Result<()> {
    let pwd = encrypt_pwd(encrypt_content)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        Err(Error::PwdNotMatching)
    }
}
