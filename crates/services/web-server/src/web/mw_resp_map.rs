use std::sync::Arc;

use super::routes_rpc::RpcInfo;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, to_value};
use uuid::Uuid;

use crate::log;
use crate::web::Error;

use super::mw_auth::CtxW;

// Client Request -> Routing, Middleware, etc. -> Server Response ->
// RES_MAPPER -> Response —> Clientl
/// Maps server error stored in extensions to client errors and returns them as responses.
pub async fn mw_response_mapper(
    ctx: Option<CtxW>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    tracing::debug!("{:<12} - mw_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // Get Rpc Info from response extensions.
    // REF: rpc::mod::rpc_handler()
    let rpc_info = res
        .extensions()
        .get::<Arc<RpcInfo>>()
        .map(|rpc| rpc.as_ref());

    // Get the eventual response error.
    let web_error = res.extensions().get::<Arc<Error>>().map(|we| we.as_ref());
    let client_status_error = web_error.map(Error::client_status_and_error);

    let error_response = client_status_error.as_ref().map(|(st_code, cl_err)| {
        let client_error = to_value(cl_err).ok();
        let message = client_error.as_ref().and_then(|v| v.get("message"));
        let detail = client_error.as_ref().and_then(|v| v.get("detail"));

        let client_error_body = json!({
            "id": rpc_info.as_ref().map(|rpc| rpc.id.clone()),
            "error": {
                "message": message, // Variant name
                "data": {
                    "req_uuid": uuid.to_string(),
                    "detail": detail,
                }
            }
        });
        tracing::debug!("CLIENT ERROR BODY: {client_error_body}");
        (*st_code, Json(client_error_body)).into_response()
    });

    //  Build and log the server log line
    let client_error = client_status_error.unzip().1;
    // TODO: Should handle errors
    #[allow(clippy::redundant_pattern_matching)]
    if let Ok(_) = log::log_request(
        uuid,
        req_method,
        uri,
        rpc_info,
        ctx,
        web_error,
        client_error,
    )
    .await
    {}
    // Either returns the CLIENT ERROR converted from SERVER ERROR,
    // or just returns unmodified response.
    error_response.unwrap_or(res)
}
