use alloc::{format, string::String, vec::Vec};

use crate::types::rpc;

pub struct RpcError {
    pub code: i64,
    pub message: String,
    pub data: Vec<u8>,
}

impl RpcError {
    pub fn not_found() -> Self {
        Self {
            code: -80002,
            message: String::from("method mot found"),
            data: Vec::new(),
        }
    }

    pub fn to_response(self) -> rpc::Response {
        rpc::Response {
            code: self.code,
            message: self.message,
            data: self.data,
        }
    }
}

impl From<alloc::string::FromUtf8Error> for RpcError {
    fn from(e: alloc::string::FromUtf8Error) -> Self {
        Self {
            code: -80001,
            message: format!("Parse utf8 error: {:?}", e),
            data: Vec::new(),
        }
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for RpcError {
    fn from(e: serde_json::Error) -> Self {
        Self {
            code: -80101,
            message: format!("json ser/de error: {:?}", e),
            data: Vec::new(),
        }
    }
}

pub type RpcResult<T> = core::result::Result<T, RpcError>;
