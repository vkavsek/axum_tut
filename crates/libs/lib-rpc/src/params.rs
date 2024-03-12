//! Base constructs for the typed RPC Params that will be used in their respective RPC handler
//! functions (e.g., `task_rpc::create_task`).
//!
//! Most of these constructs are generic over the data they hold, allowing each RPC handler
//! function to receive the type it requires.
//!

use modql::filter::ListOptions;
use serde::{de::DeserializeOwned, Deserialize};
use serde_with::{serde_as, OneOrMany};

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    pub data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    pub id: i64,
    pub data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
    pub id: i64,
}

#[serde_as]
#[derive(Deserialize)]
pub struct ParamsList<F>
where
    F: DeserializeOwned,
{
    /// Enables us to pass-in a single filter or an array of filters.
    #[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
    pub filters: Option<Vec<F>>,
    pub list_options: Option<ListOptions>,
}
