use crate::{
    crypt::pwd,
    ctx::Ctx,
    model::{
        user::{UserBmc, UserForLogin},
        ModelManager,
    },
};

use super::{Error, Result};
use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .with_state(mm)
}

/// We can use the result here because the Error that we provided implements IntoResponse trait just like Json<T>.
/// This handler also sets an 'auth-token' for the current user.
async fn api_login(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("->> {:<12} - api_login", "HANDLER");

    let LoginPayload {
        username,
        pwd: pwd_clear,
    } = payload;
    let root_ctx = Ctx::root_ctx();

    // Get the user
    let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;

    // Validate the password
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };
    pwd::validate_pwd(
        &crate::crypt::EncryptContent {
            content: pwd_clear,
            salt: user.pwd_salt.to_string(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    // FIXME: Implement real auth-token generation/signature.
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
