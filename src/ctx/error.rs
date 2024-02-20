pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr, serde::Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Auth Errors
    CtxNotInRequestExtension,
    NoAuthTokenCookie,
    WrongTokenFormat,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
