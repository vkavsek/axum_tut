use std::{fmt::Display, str::FromStr};

use super::{Error, Result};
use crate::{
    config,
    utils::{b64u_decode, b64u_encode},
};

// Token Type
/// String format: ```ident_b64u.exp_b64u.sign_b64u```
pub struct Token {
    pub ident: String,     // Identifier (username for example).
    pub exp: String,       // Expiration date in Rfc 3339.
    pub sign_b64u: String, // Signature, base64url encoded.
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let split_input: Vec<&str> = token_str.split('.').collect();
        if split_input.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (split_input[0], split_input[1], split_input[2]);

        Ok(Token {
            ident: b64u_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeIdent)?,
            exp: b64u_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,
            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

// Web Token Gen and Validation
pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
    let config = config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)
}

// (private) Token Gen and Validation
fn _generate_token(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> {
    todo!()
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
    todo!()
}

/// Create token signature from token parts and salt.
fn _token_sign_into_b64u(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[serial_test::serial]
    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token_str = "ZnhfaWRlbnRfMDE.MjAyNC0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
        let fx_token = Token {
            ident: "fx_ident_01".to_string(),
            exp: "2024-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sign-b64u-encoded".to_string(),
        };

        assert_eq!(fx_token.to_string(), fx_token_str);

        Ok(())
    }
}
