mod error;
pub mod mw_auth;
pub mod mw_resp_map;
pub mod routes_login;
pub mod routes_static;
pub mod routes_tickets;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

pub const AUTH_TOKEN: &str = "auth-token";
