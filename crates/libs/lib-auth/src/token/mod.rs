mod error;

use hmac::{Hmac, Mac};
use lib_utils::{
    b64::{b64u_decode_to_string, b64u_encode},
    time::{now_utc, now_utc_plus_sec_str, parse_utc},
};
use sha2::Sha512;
use uuid::Uuid;

use super::auth_config;

pub use self::error::{Error, Result};

use std::{fmt::Display, str::FromStr};

/// Token Type
/// String format: ```ident_b64u.exp_b64u.sign_b64u```
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
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
            return Err(Error::InvalidFormat);
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (split_input[0], split_input[1], split_input[2]);

        Ok(Token {
            ident: b64u_decode_to_string(ident_b64u).map_err(|_| Error::CannotDecodeIdent)?,
            exp: b64u_decode_to_string(exp_b64u).map_err(|_| Error::CannotDecodeExp)?,
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
pub fn generate_web_token(user: &str, salt: Uuid) -> Result<Token> {
    let config = auth_config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: Uuid) -> Result<()> {
    let config = auth_config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)
}

// (private) Token Gen and Validation
fn _generate_token(ident: &str, duration_sec: f64, salt: Uuid, key: &[u8]) -> Result<Token> {
    let exp = now_utc_plus_sec_str(duration_sec);

    let sign_b64u = _token_sign_into_b64u(ident, &exp, salt, key)?;

    Ok(Token {
        ident: ident.to_string(),
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: Uuid, key: &[u8]) -> Result<()> {
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

    // Validate signature
    if new_sign_b64u != origin_token.sign_b64u {
        return Err(Error::SignatureNotMatching);
    }
    // Validate Expiration
    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| Error::ExpNotIso)?;
    let now = now_utc();
    if origin_exp < now {
        return Err(Error::Expired);
    }

    Ok(())
}

/// Create token signature from token parts and salt.
fn _token_sign_into_b64u(ident: &str, exp: &str, salt: Uuid, key: &[u8]) -> Result<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

    let mut hmac_sha512 =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::HmacFailNewFromSlice)?;

    // Add content
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = b64u_encode(result_bytes);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

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
    #[test]
    fn test_token_from_str_ok() -> Result<()> {
        let fx_token_str = "ZnhfaWRlbnRfMDE.MjAyNC0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
        let fx_token = Token {
            ident: "fx_ident_01".to_string(),
            exp: "2024-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sign-b64u-encoded".to_string(),
        };

        let token: Token = fx_token_str.parse()?;

        assert_eq!(token, fx_token);

        Ok(())
    }
    #[test]
    fn test_validate_web_token_ok() -> Result<()> {
        let fx_user = "user";
        let salt = Uuid::parse_str("d9c8f2e5-9c2d-4d11-9f73-6f75e073a362").unwrap();
        let fx_duration_sec = 0.02; // 20ms
        let token_key = &auth_config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user, fx_duration_sec, salt, token_key)?;

        thread::sleep(Duration::from_millis(10));

        validate_web_token(&fx_token, salt)
    }
    #[test]
    fn test_validate_web_token_err_expired() -> Result<()> {
        let fx_user = "user";
        let salt = Uuid::parse_str("7223e4fe-bc44-4ddc-be5e-8d467b28b940").unwrap();
        let fx_duration_sec = 0.01; // 20ms
        let token_key = &auth_config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user, fx_duration_sec, salt, token_key)?;

        thread::sleep(Duration::from_millis(20));

        let res = validate_web_token(&fx_token, salt);
        assert!(
            matches!(res, Err(Error::Expired)),
            "Should have matched 'Err(Error::TokenExpired)' but was '{res:?}'"
        );

        Ok(())
    }
}
