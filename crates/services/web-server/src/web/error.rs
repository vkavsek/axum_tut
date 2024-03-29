use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use lib_auth::token;
use lib_core::model;
use serde::Serialize;

use super::mw_auth;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr, From)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd {
        user_id: i64,
    },
    LoginFailPwdNotMatching {
        user_id: i64,
    },

    // -- CtxExtError
    #[from]
    CtxExt(mw_auth::CtxExtError),

    // -- Modules
    #[from]
    Model(model::Error),
    #[from]
    Token(token::Error),

    // -- Libs
    #[from]
    Rpc(lib_rpc::Error),
}

// Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

/// From the root error to the http status code and ClientError
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use self::Error::*;

        #[allow(unreachable_patterns)]
        match self {
            // -- Login
            LoginFailPwdNotMatching { .. }
            | LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. } => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // -- Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // -- Model
            Model(model::Error::EntityNotFound { entity, id }) => (
                StatusCode::BAD_REQUEST,
                ClientError::ENTITY_NOT_FOUND { entity, id: *id },
            ),

            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
    SERVICE_ERROR,
}
