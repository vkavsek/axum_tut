use crate::ctx::Ctx;
use crate::log::log_request;
use crate::web;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, to_value};
use uuid::Uuid;

// Client Request -> Routing, Middleware, etc. -> Server Response ->
// RES_MAPPER -> Response —> Clientl
/// Maps server error stored in extensions to client errors and returns them as responses.
pub async fn mw_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    tracing::debug!("{:<12} - mw_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // Get the eventual response error.
    let web_error = res.extensions().get::<web::Error>();
    let client_status_error = web_error.map(web::Error::client_status_and_error);

    let error_response = client_status_error.as_ref().map(|(st_code, cl_err)| {
        let client_error = to_value(cl_err).ok();
        let message = client_error.as_ref().and_then(|v| v.get("message"));
        let detail = client_error.as_ref().and_then(|v| v.get("detail"));

        let client_error_body = json!({
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
    if let Ok(_) = log_request(uuid, req_method, uri, ctx, web_error, client_error).await {}
    // Either returns the CLIENT ERROR converted from SERVER ERROR,
    // or just returns unmodified response.
    error_response.unwrap_or(res)
}
