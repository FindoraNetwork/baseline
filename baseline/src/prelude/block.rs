use async_trait::async_trait;

use crate::{types::ExecResults};

use super::{ContextMut, Mempool, Genesis};

use alloc::boxed::Box;

#[async_trait]
pub trait Block: Mempool + Genesis {
    async fn apply_txs(&mut self, _tx: &[Self::Transaction]) -> ExecResults
    where
        Self::Context: ContextMut,
    {
        Default::default()
    }
}
