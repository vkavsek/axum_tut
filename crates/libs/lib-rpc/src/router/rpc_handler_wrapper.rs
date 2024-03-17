use futures::Future;
use serde_json::Value;
use std::{marker::PhantomData, pin::Pin};

use super::{PinFutureValue, RpcHandler};
use crate::{resources::RpcResources, Result};

#[derive(Debug)]
/// `RpcHandlerWrapper` type exists so we can wrap any handler that implements `RpcHandler`.
/// We provide an implementation of `call` through which we call the appropriate RPC method.
/// This way we can avoid monomorphization, which is desired if we want to scale our available RPC
/// methods.
pub struct RpcHandlerWrapper<H, T, P, R> {
    handler: H,
    _marker: PhantomData<(T, P, R)>,
}

impl<H, T, P, R> RpcHandlerWrapper<H, T, P, R> {
    /// Constructor
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

/// A blanket implementation of this wrapper for any handler that implements `RpcHandler`.
/// Here we provide the `call` method that will dynamically call the appropriate RPC method.
impl<H, T, P, R> RpcHandlerWrapper<H, T, P, R>
where
    H: RpcHandler<T, P, R> + Send + Sync + 'static,
    T: Send + Sync + 'static,
    P: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    pub fn call(&self, rpc_resources: RpcResources, params: Option<Value>) -> H::Future {
        // NOTE: Since handler is `FnOnce` we can use it only once, so we clone it.
        // This is likely optimized by the compiler.
        let handler = self.handler.clone();
        handler.call(rpc_resources, params)
    }
}

/// A trait that enables us to dynamically store any handler that implements `RpcHandler` in the
/// same `RpcRouter` (uses HashMap under the hood).
pub trait RpcHandlerWrapperTrait: Send + Sync {
    fn call(
        &self,
        rpc_resources: RpcResources,
        params: Option<Value>,
    ) -> Pin<Box<dyn Future<Output = Result<Value>> + Send>>;
}

/// A blanket implementation of the `RpcHandlerWrapperTrait`
impl<H, T, P, R> RpcHandlerWrapperTrait for RpcHandlerWrapper<H, T, P, R>
where
    H: RpcHandler<T, P, R> + Send + Sync + 'static,
    T: Send + Sync + 'static,
    P: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    fn call(&self, rpc_resources: RpcResources, params: Option<Value>) -> PinFutureValue {
        Box::pin(self.call(rpc_resources, params))
    }
}
