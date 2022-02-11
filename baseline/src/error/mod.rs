pub enum MempoolError {}

pub type MempoolResult<T> = core::result::Result<T, MempoolError>;

pub enum BlockError {}

pub type BlockResult<T> = core::result::Result<T, BlockError>;

pub enum RpcError {}

pub type RpcResult<T> = core::result::Result<T, RpcError>;
