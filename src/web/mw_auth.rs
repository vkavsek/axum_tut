use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

/// Requires the client to have the correct authentication cookies in order to allow certain actions.
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}


/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
/// TODO: Take reference as input?
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::WrongTokenFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::WrongTokenFormat)?;

    Ok((user_id, exp.to_owned(), sign.to_owned()))
}

/// Context Extractor
/// Implementing a custom extractor for the Ctx struct
/// With this we can extract Ctx in our middleware implementation like demonstrated above in
/// mw_require_auth function.
#[async_trait]
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
