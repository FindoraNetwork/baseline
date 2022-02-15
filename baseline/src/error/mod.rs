pub enum MempoolError {}

pub type MempoolResult<T> = core::result::Result<T, MempoolError>;

// pub enum BlockError {}
//
// pub type BlockResult<T> = core::result::Result<T, BlockError>;

mod rpc;
pub use rpc::*;
