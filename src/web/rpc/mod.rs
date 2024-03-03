mod task_rpc;

use axum::{extract::State, response::Response, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::Value;
use tracing::debug;

use crate::{ctx::Ctx, model::ModelManager};

use super::{Error, Result};

/// JSON-RPC Request Body.
#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
    id: i64,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    unimplemented!();
}

async fn _rpc_handler(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;
    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        // Task RPC methods
        "create_task" => todo!(),
        "get_task" => todo!(),
        "list_tasks" => todo!(),
        "update_task" => todo!(),
        "delete_task" => todo!(),
        // Fallback
        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    todo!()
}
