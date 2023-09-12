#![allow(unused)]

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::{
    web::{self, AUTH_TOKEN},
    Error, Result,
};

/// Context Extractor
/// When used within a handler, this function returns a Result<Ctx>.
/// Its job is to extract cookies and search for the 'auth-token.'.
/// If it locates the token, it attempts to parse it into a valid token and
/// then returns the Result<Ctx{ user_id}, Error>.
#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: u64,
}
impl Ctx {
    pub fn from(user_id: u64) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

// Implementing a custom extractor for the Ctx struct
// Note the async_trait macro! It is necessary when we want to implement Async traits.
#[async_trait::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Ctx> {
        println!("->> {:<12} — Ctx", "EXTRACTOR");

        // Uses the cookies extractor
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, _exp, _sing) = auth_token
            .ok_or(Error::NoAuthTokenCookie)
            .and_then(parse_token)?;

        // TODO —> Token components validation

        Ok(Ctx::from(user_id))
    }
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
/// TODO: Take reference as input?
pub fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::WrongTokenFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::WrongTokenFormat)?;

    Ok((user_id, exp.to_owned(), sign.to_owned()))
}
