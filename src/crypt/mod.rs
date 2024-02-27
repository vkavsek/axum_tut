mod error;
pub mod pwd;

use hmac::{Hmac, Mac};
use sha2::Sha512;

pub use self::error::{Error, Result};

pub struct EncryptContent {
    pub content: String, // Clear content
    pub salt: String,    // Clear salt.
}

/// Encrypt into Base64-URL
pub fn encrypt_into_b64u(key: &[u8], to_encrypt: &EncryptContent) -> Result<String> {
    let EncryptContent { content, salt } = to_encrypt;

    // Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

    // Add content
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = base64_url::encode(&result_bytes);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rand::RngCore;

    #[serial_test::serial]
    #[tokio::test]
    async fn test_encrypt_b64u_ok() -> Result<()> {
        let mut fx_key = [0u8; 64]; // 512 bits == 64 bytes
        rand::thread_rng().fill_bytes(&mut fx_key);
        let fx_to_encrypt = EncryptContent {
            content: "hello world".into(),
            salt: "random".into(),
        };

        // TODO: Need to have a fixed fx_key, and precompute fx_res.
        let fx_res = encrypt_into_b64u(&fx_key, &fx_to_encrypt)?;
        let res = encrypt_into_b64u(&fx_key, &fx_to_encrypt)?;

        assert_eq!(fx_res, res);

        Ok(())
    }
}
