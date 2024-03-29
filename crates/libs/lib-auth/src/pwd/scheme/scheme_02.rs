//! Argon2 Hashing Scheme
use std::sync::OnceLock;

use argon2::{
    password_hash::SaltString, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::config::auth_config;

use super::{Error, Result, Scheme};

pub struct Scheme02;

impl Scheme for Scheme02 {
    fn hash(&self, to_hash: &crate::pwd::ContentToHash) -> Result<String> {
        let argon2 = get_argon2();

        let salt_b64 = SaltString::encode_b64(to_hash.salt.as_bytes()).map_err(|_| Error::Salt)?;

        let pwd = argon2
            .hash_password(to_hash.content.as_bytes(), &salt_b64)
            .map_err(|_| Error::Hash)?
            .to_string();

        Ok(pwd)
    }

    fn validate(&self, to_hash: &crate::pwd::ContentToHash, raw_pwd_ref: &str) -> Result<()> {
        let argon2 = get_argon2();

        let parsed_hash_ref = PasswordHash::new(raw_pwd_ref).map_err(|_| Error::Hash)?;

        argon2
            .verify_password(to_hash.content.as_bytes(), &parsed_hash_ref)
            .map_err(|_| Error::PwdValidate)
    }
}

fn get_argon2() -> &'static Argon2<'static> {
    static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let key = &auth_config().PWD_KEY;
        Argon2::new_with_secret(
            &key,
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::default(),
        )
        .unwrap() // TODO: needs to fail early
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pwd::ContentToHash;
    use anyhow::Result;
    use uuid::Uuid;

    #[test]
    fn test_scheme_02_hash_into_b64u_ok() -> Result<()> {
        let fx_to_hash = ContentToHash {
            content: "hello".into(),
            salt: Uuid::parse_str("38f77f0d-7c7e-43ef-a48d-7f4c53e51779")?,
        };
        // TODO: Need to fix fx_key and precompute fx_res
        let fx_res = "$argon2id$v=19$m=19456,t=2,p=1$OPd/DXx+Q++kjX9MU+UXeQ$A/BusuzlHACUihIEgB21NtXPtVvhAuVX5bL3N9l9b18";

        let s02 = Scheme02;
        let res = s02.hash(&fx_to_hash)?;
        assert_eq!(fx_res, res);

        Ok(())
    }
}
