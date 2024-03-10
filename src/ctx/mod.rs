#![allow(unused)]

mod error;
pub use self::error::{Error, Result};

/// Context Extractor
/// When used within a handler, `Ctx` implements `FromRequestParts`.
/// `from_request_parts` function returns a `Result<Ctx>`.
/// Its job is to extract cookies and search for the 'auth-token.'.
/// If it locates the token, it attempts to parse it into a valid token and
/// then returns the `Result<Ctx{ user_id}, Error>`.
#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: i64,
}
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }
    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
