mod rpc;
pub use rpc::*;

use alloc::string::String;

// pub enum MempoolError {}
//
// pub type MempoolResult<T> = core::result::Result<T, MempoolError>;
//
#[derive(Debug, Default, Clone)]
pub struct Error {
    pub code: i64,
    pub message: String,
    pub codespace: String,
}

pub type Result<T> = core::result::Result<T, Error>;
