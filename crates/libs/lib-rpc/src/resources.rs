use lib_core::{ctx::Ctx, model::ModelManager};

use crate::{router::FromResources, Result};

pub struct RpcResources {
    pub mm: ModelManager,
    pub ctx: Option<Ctx>,
}

impl FromResources for Ctx {
    fn from_resources(rpc_resources: &RpcResources) -> Result<Self>
    where
        Self: Sized,
    {
        rpc_resources
            .ctx
            .as_ref()
            .cloned()
            .ok_or(crate::Error::MissingCtx)
    }
}

impl FromResources for Option<Ctx> {
    fn from_resources(rpc_resources: &RpcResources) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(rpc_resources.ctx.as_ref().cloned())
    }
}

impl FromResources for ModelManager {
    fn from_resources(rpc_resources: &RpcResources) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(rpc_resources.mm.clone())
    }
}
