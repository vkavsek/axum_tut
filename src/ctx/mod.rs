#![allow(unused)]

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use tower_cookies::Cookies;

use crate::{
    web::{self, AUTH_TOKEN},
    Result,
};

pub mod error;
pub use self::error::Error;

/// Context Extractor
/// When used within a handler, Ctx implements FromRequestParts.
/// `from_request_parts` function returns a Result<Ctx>.
/// Its job is to extract cookies and search for the 'auth-token.'.
/// If it locates the token, it attempts to parse it into a valid token and
/// then returns the Result<Ctx{ user_id}, Error>.
#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: u64,
}
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

// NOTE: the async_trait macro! It is necessary when we want to implement Async traits.
#[async_trait::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        tracing::debug!("->> {:<12} — Ctx", "EXTRACTOR");

        // TODO: Token components validation
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::CtxNotInRequestExtension)?
            .clone()
    }
}
