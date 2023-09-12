use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::ctx::Ctx;
use crate::Result;

/// Requires the client to have the correct authentication cookies in order to allow certain actions.
/// It achieves that by using the Ctx extractor. 
/// Check out Ctx documentation for more details.
/// Because we expand the errors here, we can simply use Ctx (not Result<Ctx>) in all the handlers
/// that run after the authentication.
/// in this case all the handlers inside of '/api/' path.
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}
