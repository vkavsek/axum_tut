use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use super::{Error, Result, AUTH_TOKEN};
use crate::crypt::token::{self, Token};
use crate::ctx::Ctx;
use crate::model::user::{UserBmc, UserForAuth};
use crate::web::set_token_cookie;
use crate::ModelManager;

/// Ctx Extractor Result
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

/// Ctx Extractor Error
#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenWrongFormat,

    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokenCookie,

    CtxNotInRequestExt,
    CtxCreateFail(String),
}

/// All the middleware runs only once per request.
/// Here we do all the heavy lifting:
/// token parsing, token components validation, etc.
/// Then we store the Ctx into request extensions so it can be retrieved by the Ctx Extractor.
/// If we do all those things in the extractor it can get expensive since an extractor runs
/// everytime a handler calls it, that means that it can run multiple times per-request.
pub async fn mw_ctx_resolve<B>(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    tracing::debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

    // Resolve Ctx
    let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

    // Remove the cookie if Ctx resolve was invalid
    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    // Insert the result of Ctx creation into request extensions
    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: ModelManager, cookies: &Cookies) -> CtxExtResult {
    // Get token string
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    // Parse token (FromStr)
    let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

    // Get UserForAuth
    let user: UserForAuth = UserBmc::first_by_username(&Ctx::root_ctx(), &mm, &token.ident)
        .await
        .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    // Validate token
    token::validate_web_token(&token, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::FailValidate)?;

    // Update Token
    set_token_cookie(cookies, &user.username, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

    // Create CtxExt Result
    Ctx::new(user.id).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

/// Ctx Extractor
/// Runs for each handler that runs in a request - multiple times per request.
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

/// Requires the client to have the correct authentication cookies in order to allow certain actions.
/// It achieves that by using the Ctx extractor.
/// Check out Ctx documentation for more details.
/// Because we expand the errors here, we can simply use Ctx (not `Result<Ctx>`) in all the handlers
/// that run after the authentication - any route that relies on `mw_ctx_require` 'knows' that the
/// Ctx will be in the extensions - in this case all the handlers inside of '/api/' path.
/// The extractor function `from_request_parts` still runs on every extraction.
pub async fn mw_ctx_require<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    tracing::debug!("{:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}
