use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    uname: String,
    pass: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

/// We can use the result here because the Error that we provided implements IntoResponse trait just like Json<T>.
/// This handler also sets an 'auth-token' for the current user.
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO - implement real db/auth logic
    if payload.uname != "demo1" || payload.pass != "1234" {
        return Err(Error::LoginFail);
    }

    // TODO - Implement real auth-token generation/signature.
    cookies.add(Cookie::new(super::AUTH_TOKEN, "user-1.exp.sign"));

    // Success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }
    ));
    Ok(body)
}
