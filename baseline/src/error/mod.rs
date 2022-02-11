use alloc::{string::String, vec::Vec, format};

pub enum MempoolError {}

pub type MempoolResult<T> = core::result::Result<T, MempoolError>;

pub enum BlockError {}

pub type BlockResult<T> = core::result::Result<T, BlockError>;

pub struct RpcError {
    pub code: i64,
    pub message: String,
    pub data: Vec<u8>,
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for RpcError {
    fn from(e: serde_json::Error) -> Self {
        Self {
            code: -80001,
            message: format!("json ser/de error: {:?}", e),
            data: Vec::new(),
        }
    }
}

pub type RpcResult<T> = core::result::Result<T, RpcError>;
