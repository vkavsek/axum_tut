use serde_json::Value;
use std::marker::PhantomData;

use super::{PinFutureValue, RpcHandler};
use crate::resources::RpcResources;

#[derive(Debug)]
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

pub trait RpcHandlerWrapperTrait: Send + Sync {
    fn call(&self, rpc_resources: RpcResources, params: Option<Value>) -> PinFutureValue;
}

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
