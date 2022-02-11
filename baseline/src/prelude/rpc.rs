use alloc::{string::String, vec::Vec};
use async_trait::async_trait;

use crate::{error::RpcResult, types::rpc};

use super::Module;

#[async_trait]
pub trait RPC: Module {
    fn call(&mut self, req: rpc::Request) -> RpcResult<rpc::Response>;
}

pub trait Requester: Sized {
    fn request(req: &rpc::Request) -> RpcResult<Self>;
}

impl Requester for String {
    fn request(req: &rpc::Request) -> RpcResult<Self> {
        Ok(String::from_utf8(req.params)?)
    }
}

impl Requester for Vec<u8> {
    fn request(req: &rpc::Request) -> RpcResult<Self> {
        Ok(req.params)
    }
}

impl<const N: usize> Requester for [u8; N] {
    fn request(req: &rpc::Request) -> RpcResult<Self> {
        let mut req = [0u8; N];

        // let

        Ok(req)
    }
}


pub trait Responder {
    fn response(self) -> RpcResult<rpc::Response>;
}

