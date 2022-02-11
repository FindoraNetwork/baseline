use async_trait::async_trait;

use crate::{error::RpcResult, types::rpc};

use super::Module;

#[async_trait]
pub trait RPC: Module {
    fn call(&mut self, req: rpc::Request) -> RpcResult<rpc::Response>;
}
