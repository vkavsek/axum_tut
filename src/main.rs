use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;
//#[cfg(test)] // Commented during early dev.
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use crate::{
    model::ModelManager,
    web::{mw_auth, mw_resp_map::mw_response_mapper, routes_login, routes_static},
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // For early local dev.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env()) // You can setup the enviroment variables in .cargo/config.toml
        .init();

    // FOR DEV ONLY
    _dev_utils::init_dev().await;

    // init ModelManager
    let mm = ModelManager::init().await?;

    // route_layer() adds middleware to existing routes. You first have to add your routes!
    // This will only run if the request matches a route, in this case: "/api/tickets".
    // That means that other routers won't be impacted by this middleware.
    // let routes_apis = routes_tickets::routes(mm.clone())
    //     .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    // .merge() allows to compose many routers together.
    // .fallback_service() falls back to the static render.
    // The .layer() gets executed from bottom to top, so if you want other layers to have
    // Cookie data the CookieManagerLayer needs to be on the bottom.
    let routers = Router::new()
        .merge(routes_login::routes(mm.clone()))
        // .nest("/api", routes_apis)
        .layer(middleware::map_response(mw_response_mapper))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    // ————>        START SERVER
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("-->> LISTENING on {}\n", addr);
    axum::Server::bind(&addr)
        .serve(routers.into_make_service())
        .await
        .unwrap();

    Ok(())
}
