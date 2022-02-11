use async_trait::async_trait;

use crate::{error::BlockResult, types::ExecResults};

use super::{ContextMut, Mempool};

use alloc::boxed::Box;

#[async_trait]
pub trait Block: Mempool {
    async fn apply_txs(&mut self, _tx: &[Self::Transaction]) -> BlockResult<ExecResults>
    where
        Self::Context: ContextMut,
    {
        Ok(Default::default())
    }
}
