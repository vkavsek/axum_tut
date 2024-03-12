use axum::response::IntoResponse;
use axum::{extract::State, response::Response, routing::post, Json, Router};
use lib_core::{ctx::Ctx, model::ModelManager};
use lib_rpc::{exec_rpc, RpcInfo, RpcRequest};
use serde_json::{json, Value};
use tracing::debug;

use super::Result;

use super::mw_auth::CtxW;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: CtxW,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    // Create the RPC Info to be set to the response extensions.
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    // Exec & Store RpcInfo in response.
    let mut response = _rpc_handler(ctx.0, mm, rpc_req).await.into_response();

    response.extensions_mut().insert(rpc_info);

    response
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let rpc_method = rpc_req.method.clone();
    let rpc_id = rpc_req.id.clone();
    debug!("{:<12} - rpc_handler - method: {rpc_method}", "HANDLER");

    let result = exec_rpc(ctx, mm, rpc_req).await?;

    let body_res = json!({
        "id": rpc_id,
        "result": result,
    });

    Ok(Json(body_res))
}
