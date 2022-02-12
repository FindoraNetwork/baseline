#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use json::*;

pub mod helpers {
    use crate::{prelude::Responder, types::rpc::Response, RpcResult};

    pub fn to_response(r: RpcResult<Response>) -> Response {
        match r {
            Ok(r) => r,
            Err(e) => e.to_response(),
        }
    }

    pub fn to_result_response<T: Responder>(r: RpcResult<T>) -> RpcResult<Response> {
        r.map(|t| to_response(t.response()))
    }
}
