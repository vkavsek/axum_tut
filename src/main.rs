#![allow(unused)]

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router = Router::new().merge(routes_hello());

    //          ---> START SERVER
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-->> LISTENING on {}\n", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    //          <--- START SERVER
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
    Html(format!("Hello <i>{}!!</i>", name))
}
