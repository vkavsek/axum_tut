//! rpc::router module provides the types for and implementation of json-rpc routing
//!
//! It contains the following constructs:
//!
//! - `RpcRouter` holds the Hashmap of `method_name: Box<dyn RpcHandlerWrapperTrait>`.
//! - `RpcHandler` trait is implemented for any async function that, with `(S1, S2, ... [impl IntoParams])`
//! (see router/from_resources.rs and src/resources.rs).
//! - `IntoParams` is the trait to implement to get the conversion from `Option<Value>` json-rpc
//! params to the handler's Param types.
//! - `IntoParams` has a default `into_params` implementation taht wil return an error if the
//! params are missing.
//! ```
//! #[derive(Deserialize)]
//! pub struct ParamsIded {
//!   id: i64,
//! }
//!
//! impl IntoParams for ParamsIded {}
//! ```
//! - Implement `IntoParams::into_params` function for custom behavior.
//! - Implementing `IntoDefaultParams` on a type that implements `Default` will auto implement
//! `IntoParams` and call `T::default()` when the params `Option<Value>` is None.

use std::{collections::HashMap, pin::Pin};

use futures::Future;
use serde::Deserialize;
use serde_json::Value;

use crate::{resources::RpcResources, Error, Result};

mod from_resources;
mod into_params;
mod rpc_handler;
mod rpc_handler_wrapper;

pub use from_resources::FromResources;
pub use into_params::{IntoDefaultParams, IntoParams};
pub use rpc_handler::RpcHandler;
pub use rpc_handler_wrapper::{RpcHandlerWrapper, RpcHandlerWrapperTrait};

/// The raw JSON-RPC Request Body. Serving as the foundation for RPC routing
#[derive(Deserialize)]
pub struct RpcRequest {
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

pub type PinFutureValue = Pin<Box<dyn Future<Output = Result<Value>> + Send>>;

// Method which calls the appropriate handler matching the method name.
// `RpcRouter` can be extended with other `RpcRouters` for composability.
pub struct RpcRouter {
    route_by_name: HashMap<&'static str, Box<dyn RpcHandlerWrapperTrait>>,
}

impl RpcRouter {
    pub fn init() -> Self {
        Self {
            route_by_name: HashMap::new(),
        }
    }

    /// Add a dyn handler to the router
    /// ```
    /// RpcRouter::init().add_dyn("method_name", my_handler_fn.into_dyn());
    /// ```
    /// NOTE: This is the preferred way to add handlers to the router, as it avoids
    ///       monomorphization of the add function.
    ///       The RpcRouter also has an `.add()` as a convenience function to just pass the function.
    pub fn add_dyn(
        mut self,
        name: &'static str,
        dyn_handler: Box<dyn RpcHandlerWrapperTrait>,
    ) -> Self {
        self.route_by_name.insert(name, dyn_handler);
        self
    }

    /// Add a handler function to the router.
    /// ```
    /// RpcRouter::init().add("method_name", my_handler_fn);
    /// ```
    /// NOTE: This is a convenient add function variant with generics.
    ///       There will be monomorphized versions of this function for each type passed.
    ///       Use `RpcRouter::add_dyn` to avoid this.
    pub fn add<F, T, P, R>(self, name: &'static str, handler: F) -> Self
    where
        F: RpcHandler<T, P, R> + Send + Sync + Clone + 'static,
        T: Send + Sync + 'static,
        P: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.add_dyn(name, handler.into_dyn())
    }

    pub fn extend(mut self, other_router: RpcRouter) -> Self {
        self.route_by_name.extend(other_router.route_by_name);
        self
    }

    pub async fn call(
        &self,
        method: &str,
        rpc_resources: RpcResources,
        params: Option<Value>,
    ) -> Result<Value> {
        if let Some(route) = self.route_by_name.get(method) {
            route.call(rpc_resources, params).await
        } else {
            Err(Error::MethodUnknown(method.to_string()))
        }
    }
}

/// A macro to create a new RpcRouter and add each RpcHandler compatible function.
/// e.g.,
/// ```
/// rpc_router!(
///     create_project,
///     list_projects,
///     update_project,
///     delete_project
/// );
/// ```
/// Is equivalent to:
/// ```
/// RpcRouter::new()
///     .add_dyn("create_project", create_project.into_box())
///     .add_dyn("list_projects", list_projects.into_box())
///     .add_dyn("update_project", update_project.into_box())
///     .add_dyn("delete_project", delete_project.into_box())
/// ```
#[macro_export]
macro_rules! rpc_router {
    ($($fn_name:ident),+ $(,)?) => {
        {
            use $crate::router::{RpcHandler, RpcRouter};
            let mut router = RpcRouter::init();
            $(
                router = router.add_dyn(stringify!($fn_name), $fn_name.into_dyn());
            )+
            router
        }
    };
}
