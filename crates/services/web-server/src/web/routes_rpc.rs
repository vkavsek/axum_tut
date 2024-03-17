use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use lib_core::model::ModelManager;
use lib_rpc::router::RpcRouter;
use lib_rpc::{task_rpc, RpcRequest, RpcResources};
use serde_json::{json, Value};
use std::sync::Arc;

use super::mw_auth::CtxW;

/// The `RpcState` is the Axum State that will be used for the Axum RPC router handler.
#[derive(Clone)]
pub struct RpcState {
    pub mm: ModelManager,
}

#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

pub fn routes(rpc_state: RpcState) -> Router {
    let rpc_router = RpcRouter::init().extend(task_rpc::rpc_router());

    Router::new()
        .route("/rpc", post(rpc_axum_handler))
        .with_state((rpc_state, Arc::new(rpc_router)))
}

async fn rpc_axum_handler(
    State((rpc_state, rpc_router)): State<(RpcState, Arc<RpcRouter>)>,
    ctx: CtxW,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    let ctx = ctx.0;
    // Create the RPC Info to be set to the response extensions.
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    let rpc_method = &rpc_info.method;
    let rpc_params = rpc_req.params;
    let rpc_resources = RpcResources {
        ctx: Some(ctx),
        mm: rpc_state.mm,
    };

    // Exec Rpc Route
    let res = rpc_router.call(rpc_method, rpc_resources, rpc_params).await;

    // Build RPC Success Response
    let res = res.map(|v| {
        let body_res = json!({
            "id": rpc_info.id,
            "result": v,
        });
        Json(body_res)
    });

    // Create and update Axum Response
    let res = res.map_err(crate::web::Error::from);
    let mut res = res.into_response();
    res.extensions_mut().insert(Arc::new(rpc_info));

    res
}
