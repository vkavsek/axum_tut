use crate::log::log_request;
pub use crate::{
    error::{Error, Result},
    model::ModelController,
    web::{mw_auth, routes_login, routes_tickets},
};

use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
    routing::get_service,
    Json, Router,
};
use ctx::Ctx;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod hello;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // init ModelController
    let mc = ModelController::new().await?;

    // route_layer() adds middleware to existing routes. You first have to add your routes!
    // This will only run if the request matches a route, in this case: "/api/tickets".
    // That means that other routers won't be impacted by this middleware.
    let routes_apis = routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    // .merge() allows to compose many routers together.
    // .fallback_service() falls back to the static render.
    // The .layer() gets executed from bottom to top, so if you want other layers to have
    // Cookie data the CookieManagerLayer needs to be on the bottom.
    let routers = Router::new()
        .merge(hello::routes())
        .merge(routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // ————>        START SERVER
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-->> LISTENING on {}\n", addr);
    axum::Server::bind(&addr)
        .serve(routers.into_make_service())
        .await
        .unwrap();
    // <————        START SERVER

    Ok(())
}

// Client Request -> Routing, Middleware, etc. -> Server Response ->
// RES_MAPPER -> Response —> Client
/// Maps server error stored in extensions to client errors and returns them as responses.
async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(Error::client_status_and_error);

    let error_response = client_status_error.as_ref().map(|(st_code, cl_err)| {
        let client_error_body = json!({
            "error": {
                "type": cl_err.as_ref(),
                "req_uuid": uuid.to_string(),
            }
        });
        println!("  ->> client_error_body: {client_error_body}");
        (*st_code, Json(client_error_body)).into_response()
    });

    //  Build and log the server log line
    let client_error = client_status_error.unzip().1;
    // TODO: Should handle errors
    #[allow(clippy::redundant_pattern_matching)]
    if let Ok(_) = log_request(uuid, req_method, uri, ctx, service_error, client_error).await {}
    // Either returns the CLIENT ERROR converted from SERVER ERROR,
    // or just returns unmodified response.
    error_response.unwrap_or(res)
}

/// A fallback route that serves the './' directory.
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
