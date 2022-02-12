use alloc::{string::String, vec::Vec};
use async_trait::async_trait;

use crate::{error::RpcResult, types::rpc};

use super::Module;

#[async_trait]
pub trait RPC: Module {
    fn call(&mut self, req: rpc::Request) -> rpc::Response;
}

pub trait Requester: Sized {
    fn request(req: rpc::Request) -> RpcResult<Self>;
}

impl Requester for String {
    fn request(req: rpc::Request) -> RpcResult<Self> {
        Ok(String::from_utf8(req.params)?)
    }
}

impl Requester for Vec<u8> {
    fn request(req: rpc::Request) -> RpcResult<Self> {
        Ok(req.params)
    }
}

impl<const N: usize> Requester for [u8; N] {
    fn request(req: rpc::Request) -> RpcResult<Self> {
        let mut r = [0u8; N];

        r.copy_from_slice(&req.params);

        Ok(r)
    }
}

pub trait Responder {
    fn response(self) -> RpcResult<rpc::Response>;
}

impl Responder for String {
    fn response(self) -> RpcResult<rpc::Response> {
        let data = self.as_bytes().to_vec();

        Ok(rpc::Response::new(data))
    }
}

impl Responder for Vec<u8> {
    fn response(self) -> RpcResult<rpc::Response> {
        Ok(rpc::Response::new(self))
    }
}

impl<'a> Responder for &'a [u8] {
    fn response(self) -> RpcResult<rpc::Response> {
        Ok(rpc::Response::new(self.to_vec()))
    }
}

impl<const N: usize> Responder for [u8; N] {
    fn response(self) -> RpcResult<rpc::Response> {
        Ok(rpc::Response::new(self.to_vec()))
    }
}

