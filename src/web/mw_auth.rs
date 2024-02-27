use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

use super::{Error, Result, AUTH_TOKEN};
use crate::ctx::Ctx;
use crate::ModelManager;

// CTX RESULT AND ERROR
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}

/// Requires the client to have the correct authentication cookies in order to allow certain actions.
/// It achieves that by using the Ctx extractor.
/// Check out Ctx documentation for more details.
/// Because we expand the errors here, we can simply use Ctx (not Result<Ctx>) in all the handlers
/// that run after the authentication.
/// in this case all the handlers inside of '/api/' path.
/// The extractor function `from_request_parts` still runs on every extraction.
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    tracing::debug!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

/// All the middleware runs only once per request.
/// Here we do all the heavy lifting:
/// token parsing, token components validation, etc.
/// If we do all those things in the extractor it can get expensive since an extractor runs
/// everytime a handler calls it, that means that it can run multiple times per-request.
pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    tracing::debug!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // FIXME: Compute real CtxAuthResult<Ctx>.
    let result_ctx = Ctx::new(100).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()));

    // Remove the cookie if something went wrong other than NoAuthTokenCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    // Inserted value has to be unique by type, otherwise it gets overwritten
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

/// Ctx extractor, runs for each handler that runs in a request - multiple times per request.
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

// /// Parse a token of format `user-[user-id].[expiration].[signature]`
// /// Returns Result((user_id, expiration, signature))
// fn parse_token(token: &str) -> Result<(u64, String, String)> {
//     let (_whole, user_id, exp, sign) =
//         regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, token).ok_or(Error::WrongTokenFormat)?;
//
//     let user_id: u64 = user_id.parse().map_err(|_| Error::WrongTokenFormat)?;
//
//     Ok((user_id, exp.to_owned(), sign.to_owned()))
// }
