//! Uses [JOQL](https://joql.org/) on top of [Json-RPC 2.0](https://www.jsonrpc.org/specification).
//! Reference JOQL's site for further guidance on advanced querying.
mod error;
mod params;
mod resources;
mod rpcs;

pub mod router;

pub use self::error::{Error, Result};
pub use params::*;
pub use resources::RpcResources;
pub use router::RpcRequest;
pub use rpcs::*;

// /// RPC basic information holding the id and method for further logging.
// #[derive(Debug, Deserialize)]
// pub struct RpcInfo {
//     pub id: Option<Value>,
//     pub method: String,
// }
