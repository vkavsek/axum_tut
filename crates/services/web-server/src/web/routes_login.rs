use crate::web::{remove_token_cookie, set_token_cookie};

use super::{Error, Result};
use axum::{extract::State, routing::post, Json, Router};
use lib_auth::pwd::{self, ContentToHash, SchemeStatus};
use lib_core::{
    ctx::Ctx,
    model::{
        user::{UserBmc, UserForLogin},
        ModelManager,
    },
};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::task::spawn_blocking;
use tower_cookies::Cookies;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logoff", post(api_logoff_handler))
        .with_state(mm)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

/// We can use the result here because the `Error` that we provided implements `IntoResponse` trait just like `Json<T>`.
/// This handler also sets an 'auth-token' for the current user.
async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("{:<12} - api_login", "HANDLER");

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

    // spawn_blocking(move || {
    //     pwd::validate_pwd(
    //         &ContentToHash {
    //             content: pwd_clear.clone(),
    //             salt: user.pwd_salt,
    //         },
    //         &pwd,
    //     )
    // })
    // .await
    // .unwrap()
    // .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;
    let scheme_status = pwd::validate_pwd(
        &ContentToHash {
            content: pwd_clear.clone(),
            salt: user.pwd_salt,
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    // Update password scheme if needed
    if let SchemeStatus::Outdated = scheme_status {
        debug!("Password encrypt scheme outdated, upgrading.");
        UserBmc::update_pwd(&root_ctx, &mm, user_id, &pwd_clear).await?;
    }

    // Set web token
    set_token_cookie(&cookies, &user.username, user.token_salt)?;

    // Success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }
    ));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
    logoff: bool,
}

async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logoff_handler", "HANDLER");

    let should_logoff = payload.logoff;

    if should_logoff {
        remove_token_cookie(&cookies)?;
    }

    let body = Json(json!({
        "result": {
            "logged_off": should_logoff
        }
    }));

    Ok(body)
}
