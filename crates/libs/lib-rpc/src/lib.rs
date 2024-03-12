//! Uses [JOQL](https://joql.org/) on top of [Json-RPC 2.0](https://www.jsonrpc.org/specification).
//! Reference JOQL's site for further guidance on advanced querying.

mod error;
mod params;
mod task_rpc;

use serde::Deserialize;
use serde_json::{from_value, to_value, Value};
use tracing::debug;

use lib_core::{ctx::Ctx, model::ModelManager};

use self::error::{Error, Result};
use crate::task_rpc::{create_task, delete_task, get_task, list_tasks, update_task};

/// The raw JSON-RPC Request Body. Serving as the foundation for RPC routing
#[derive(Deserialize)]
pub struct RpcRequest {
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

/// RPC basic information holding the id and method for further logging.
#[derive(Debug, Deserialize)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

// pub fn routes(mm: ModelManager) -> Router {
//     Router::new()
//         .route("/rpc", post(rpc_handler))
//         .with_state(mm)
// }
//
// async fn rpc_handler(
//     State(mm): State<ModelManager>,
//     ctx: Ctx,
//     Json(rpc_req): Json<RpcRequest>,
// ) -> Response {
//     // Create the RPC Info to be set to the response extensions.
//     let rpc_info = RpcInfo {
//         id: rpc_req.id.clone(),
//         method: rpc_req.method.clone(),
//     };
//
//     // Exec & Store RpcInfo in response.
//     let mut response = _rpc_handler(ctx, mm, rpc_req).await.into_response();
//     response.extensions_mut().insert(rpc_info);
//
//     response
// }

macro_rules! exec_rpc_fn {
    // With Params
    ($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
        let rpc_fn_name = stringify!($rpc_params);
        let params = $rpc_params.ok_or(Error::MissingParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        let params = from_value(params).map_err(|_| Error::FailJsonParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;
        $rpc_fn($ctx, $mm, params).await.map(to_value)??
    }};
    // Without Params
    ($rpc_fn:expr, $ctx:expr, $mm: expr) => {
        $rpc_fn($ctx, $mm).await.map(to_value)??
    };
}

pub async fn exec_rpc(ctx: Ctx, mm: ModelManager, rpc_req: RpcRequest) -> Result<Value> {
    let rpc_method = rpc_req.method;
    let rpc_params = rpc_req.params;
    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        // Task RPC methods
        "create_task" => exec_rpc_fn!(create_task, ctx, mm, rpc_params),
        "get_task" => exec_rpc_fn!(get_task, ctx, mm, rpc_params),
        "list_tasks" => exec_rpc_fn!(list_tasks, ctx, mm, rpc_params),
        "update_task" => exec_rpc_fn!(update_task, ctx, mm, rpc_params),
        "delete_task" => exec_rpc_fn!(delete_task, ctx, mm, rpc_params),

        // Fallback
        _ => return Err(Error::MethodUnknown(rpc_method)),
    };

    Ok(result_json)
}
