#![allow(unused)]

pub use crate::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router, middleware,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod error; 
mod model;
mod web;

#[tokio::main]
async fn main() {

    // .merge() allows to compose many routers together.
    // .fallback_service() falls back to the static render.
    // The .layer() gets executed from top to bottom, so if you want other layers to have
    // Cookie data the CookieManagerLayer needs to be on the top.
    let routers = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
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
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

/// Create and handle routes
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/hello2/:name", get(handle_hello2))
}

/// Utility struct, the 'name' variable is important if you call it something else, say 'user' the
/// query paramters would have to change to match the route.
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

/// Handles queries and greets the user or defaults to 'World!'
/// e.g.:    '/hello?name=Luka'
async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello - {:?}", "HANDLER", params);
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <i>{}!!</i>", name))
}

/// Handles paths and greets the user or defaults to 'World!'
/// e.g.:    '/hello2/Luka'
async fn handle_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello - {:?}", "HANDLER", name);
    Html(format!("Kje si <b>{}??</b>", name))
}
