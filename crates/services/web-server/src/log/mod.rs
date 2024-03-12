use axum::http::{Method, Uri};
use lib_rpc::RpcInfo;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::web::{mw_auth::CtxW, ClientError, Error, Result};

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    rpc_info: Option<&RpcInfo>,
    ctx: Option<CtxW>,
    web_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let ctx = ctx.map(|c| c.0);

    let error_type = web_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(web_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),
        user_id: ctx.map(|c| c.user_id()),

        http_path: uri.path().to_string(),
        http_method: req_method.to_string(),

        rpc_id: rpc_info.and_then(|r| r.id.as_ref().map(|id| id.to_string())),
        rpc_method: rpc_info.map(|r| r.method.clone()),

        client_error_type: client_error.map(|ce| ce.as_ref().to_string()),
        error_type,
        error_data,
    };

    tracing::debug!("REQUEST LOG LINE: \n{}\n", json!(log_line));

    // TODO: send log line to cloud-watch type of service

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // should be iso8601

    // User and context attributes
    user_id: Option<i64>,

    // HTTP request attributes
    http_path: String,
    http_method: String,

    // RPC Info
    rpc_id: Option<String>,
    rpc_method: Option<String>,

    // Error attributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
