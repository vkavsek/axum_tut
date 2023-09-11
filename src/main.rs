pub use crate::{
    error::{Error, Result},
    model::ModelController,
    web::{routes_login, routes_tickets},
};

use axum::{middleware, response::Response, routing::get_service, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod hello;
mod model;
mod mw_auth;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // init ModelController
    let mc = ModelController::new().await?;

    // route_layer() adds middleware to existing routes. You first have to add your routes!
    // This will only run if the request matches a route, in this case: "/api/tickets".
    // That means that other routers won't be impacted by this middleware.
    let routes_apis =
        routes_tickets::routes(mc).route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    // .merge() allows to compose many routers together.
    // .fallback_service() falls back to the static render.
    // The .layer() gets executed from top to bottom, so if you want other layers to have
    // Cookie data the CookieManagerLayer needs to be on the top.
    let routers = Router::new()
        .merge(hello::routes_hello())
        .merge(routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
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

// Why is this useful?
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper\n", "RES_MAPPER");
    res
}

/// A fallback route that serves the './' directory.
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
