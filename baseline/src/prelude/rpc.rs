use async_trait::async_trait;

use crate::{types::rpc, error::RpcResult};

#[async_trait]
pub trait RPC {
    fn call(&mut self, req: rpc::Request) -> RpcResult<rpc::Response>;
}
