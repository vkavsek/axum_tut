use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{Error, Result};

/// `IntoParams` allows for converting an `Option<Value>` into the neccessary type for RPC
/// handler parameters.
/// The default implementation below will result in failure if the value is `None`.
/// Implement `into_params` for custom behavior.
pub trait IntoParams: DeserializeOwned + Send {
    fn into_params(value: Option<Value>) -> Result<Self> {
        match value {
            Some(val) => Ok(serde_json::from_value(val)?),
            None => Err(Error::RpcIntoParamsMissing),
        }
    }
}

/// Marker trait with a blanket implementation that returns T::default if the value of `params` is `None`.
pub trait IntoDefaultParams: DeserializeOwned + Send + Default {}

impl<P> IntoParams for P
where
    P: IntoDefaultParams,
{
    fn into_params(value: Option<Value>) -> Result<Self> {
        match value {
            Some(val) => Ok(serde_json::from_value(val)?),
            None => Ok(Self::default()),
        }
    }
}
