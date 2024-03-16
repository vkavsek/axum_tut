//! Base constructs for the typed RPC Params that will be used in their respective RPC handler
//! functions (e.g., `task_rpc::create_task`).
//!
//! Most of these constructs are generic over the data they hold, allowing each RPC handler
//! function to receive the type it requires.
//!
//!`IntoParams` or `IntoDefaultParams` are implemented to enxure these `Params` conform to the
//!`RpcRouter` model.

use modql::filter::ListOptions;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use serde_with::{serde_as, OneOrMany};

use crate::router::{IntoDefaultParams, IntoParams};

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    pub data: D,
}

impl<D> IntoParams for ParamsForCreate<D> where D: DeserializeOwned + Send {}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    pub id: i64,
    pub data: D,
}

impl<D> IntoParams for ParamsForUpdate<D> where D: DeserializeOwned + Send {}

#[derive(Deserialize)]
pub struct ParamsIded {
    pub id: i64,
}

impl IntoParams for ParamsIded {}

#[serde_as]
#[derive(Deserialize, Default)]
pub struct ParamsList<F>
where
    F: DeserializeOwned,
{
    /// Enables us to pass-in a single filter or an array of filters.
    #[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
    pub filters: Option<Vec<F>>,
    pub list_options: Option<ListOptions>,
}

impl<D> IntoDefaultParams for ParamsList<D> where D: DeserializeOwned + Send + Default {}

// Generic Implementation
/// Implements `IntoParams` for any type that also implements ` IntoParams`.
///
/// NOTE: Application code might prefer to avoid this blanket implementation
///       and opt for enabling it on a per-type basis instead. If that's the case,
///       simply remove this general implementation and provide specific
///       implementations for each type.
impl<D> IntoParams for Option<D>
where
    D: DeserializeOwned + Send,
    D: IntoParams,
{
    fn into_params(value: Option<serde_json::Value>) -> crate::Result<Self> {
        let value = value.map(|v| serde_json::from_value(v)).transpose()?;
        Ok(value)
    }
}

/// `IntoParams` implementation for `serde_json::Value`.
///
/// NOTE: As above, this might not be the capability you want to expose to the rpc_handlers.
///       If the preference is to have everything strongly typed this implementation could just be
///       removed.
impl IntoParams for Value {}
