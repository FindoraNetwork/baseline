use async_trait::async_trait;

use crate::{types::ExecResults, error::BlockResult};

use super::Mempool;

use alloc::boxed::Box;

#[async_trait]
pub trait Block: Mempool {
    async fn apply_txs(&mut self, _tx: &[Self::Transaction]) -> BlockResult<ExecResults> {
        Ok(Default::default())
    }
}
