//! Hmac Sha512 Hashing Scheme
use hmac::{Hmac, Mac};
use lib_utils::b64::b64u_encode;
use sha2::Sha512;

use crate::config::auth_config;

use super::{ContentToHash, Error, Result, Scheme};

pub struct Scheme01;

impl Scheme for Scheme01 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let key = &auth_config().PWD_KEY;
        hmac_sha512_hash(key, to_hash)
    }

    fn validate(&self, to_hash: &ContentToHash, raw_pwd_ref: &str) -> Result<()> {
        let raw_pwd = self.hash(to_hash)?;
        if raw_pwd == raw_pwd_ref {
            Ok(())
        } else {
            Err(Error::PwdValidate)
        }
    }
}

/// Encrypt into Base64-URL
pub fn hmac_sha512_hash(key: &[u8], to_hash: &ContentToHash) -> Result<String> {
    let ContentToHash { content, salt } = to_hash;

    // Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::Key)?;

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
    use super::*;
    use anyhow::Result;
    use uuid::Uuid;

    #[test]
    fn test_scheme_01_hash_into_b64u_ok() -> Result<()> {
        let fx_salt = Uuid::parse_str("38f77f0d-7c7e-43ef-a48d-7f4c53e51779")?;
        let fx_to_hash = ContentToHash {
            content: "hello".into(),
            salt: fx_salt,
        };

        // TODO: Need to fix fx_key and precompute fx_res
        let fx_res = "FS0NYQqbKX-QDd-Rg-PrWoYYzqECxGGNbQKZpYSWCal2gPdAjJ4-Vx6YbycawKXJEIK5oXTBTVOYXGMbBn35Tg";

        let s01 = Scheme01;
        let res = s01.hash(&fx_to_hash)?;
        assert_eq!(fx_res, res);

        Ok(())
    }
}
