use axum::{extract::State, http::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, ModelManager, Result};

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
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

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
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validations.
            Ok(Ctx::from(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    // Inserted value has to be unique by type, otherwise it gets overwritten
    let mut req = req;
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns Result((user_id, expiration, signature))
/// TODO: Take reference as input?
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthWrongTokenFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthWrongTokenFormat)?;

    Ok((user_id, exp.to_owned(), sign.to_owned()))
}
