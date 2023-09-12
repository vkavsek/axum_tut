use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::ctx::Ctx;
use crate::Result;

/// Requires the client to have the correct authentication cookies in order to allow certain actions.
/// It achieves that by using the Ctx extractor. 
/// Check out Ctx documentation for more details.
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}
