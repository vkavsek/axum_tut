use crate::{ctx, model};
use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr, serde::Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Web Errors
    LoginFail,
    // Config Errors
    ConfigMissingEnv(&'static str),

    Auth(ctx::Error),
    Model(model::Error),
}
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Error::LoginFail => (StatusCode::BAD_REQUEST, ClientError::LOGIN_FAIL),
            Error::Auth(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            Error::Model(crate::model::Error::ModelTicketIdNotFound(_)) => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl From<model::error::Error> for Error {
    fn from(value: model::error::Error) -> Self {
        Error::Model(value)
    }
}
impl From<ctx::error::Error> for Error {
    fn from(value: ctx::error::Error) -> Self {
        Error::Auth(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::debug!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
