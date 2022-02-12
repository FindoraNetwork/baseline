use serde::{Deserialize, Serialize};

use crate::{prelude::{Requester, Responder}, types, RpcResult};

pub struct Json<T>(pub T);

impl<T> Json<T> {
    pub fn new(t: T) -> Self {
        Self (t)
    }
}

impl<T> Requester for Json<T>
where T: for<'de> Deserialize<'de>,
{
    fn request(req: types::rpc::Request) -> RpcResult<Self> {
        let r = serde_json::from_slice(&req.params)?;
        Ok(Self(r))
    }
}

impl<T> Responder for Json<T>
where T: Serialize, {
    fn response(self) -> RpcResult<types::rpc::Response> {
        let r = serde_json::to_vec(&self.0)?;

        let resp = types::rpc::Response::new(r);

        Ok(resp)
    }
}

impl<T> Responder for RpcResult<Json<T>> 
where T: Serialize,
{
    fn response(self) -> RpcResult<types::rpc::Response> {
        match self {
            Ok(e) => {e.response()},
            Err(e) => {
                Ok(e.to_response())
            }
        }
    }
}

