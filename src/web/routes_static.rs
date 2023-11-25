use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

/// A fallback route that serves the './' directory.
pub fn serve_dir() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
