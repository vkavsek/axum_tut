use lib_utils::b64::b64u_encode;

use hmac::{Hmac, Mac};
use sha2::Sha512;

use super::{ContentToHash, Error, Result};

/// Encrypt into Base64-URL
pub fn hmac_sha512_hash(key: &[u8], to_hash: &ContentToHash) -> Result<String> {
    let ContentToHash { content, salt } = to_hash;

    // Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

    // Add content
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = b64u_encode(result_bytes);

    Ok(result)
}
