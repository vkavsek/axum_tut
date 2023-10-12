pub use crate::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

/// Create and handle routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(handle_hello))
        .route("/hello2/:name", get(handle_hello2))
}

/// Utility struct, the 'name' variable is important if you call it something else, say 'user' the
/// query paramters would have to change to match the route.
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

/// A basic handler that responds with a static string
async fn root() -> &'static str {
    "Hola MUNDO!"
}

/// Handles queries and greets the user or defaults to 'World!'
/// e.g.:    '/hello?name=Luka'
async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello - {:?}", "HANDLER", params);
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <i>{}!!</i>", name))
}

/// Handles paths and greets the user.
/// e.g.:    '/hello2/Luka'
async fn handle_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello - {:?}", "HANDLER", name);
    Html(format!("Kje si <b>{}??</b>", name))
}
