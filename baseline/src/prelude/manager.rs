use alloc::{boxed::Box, vec::Vec};

use crate::{
    types::{CheckResponse, ExecResults},
    Metadata, Result,
};

use super::{Context, ContextMut, OriginTransaction, Transaction};

#[async_trait::async_trait]
pub trait Manager: Send + Sync + Clone {
    // From Context
    type Context: Context;

    fn set_ctx(&mut self, context: Self::Context);

    // Module's metadata.
    fn modules(&self) -> Vec<Metadata>;

    // From Mempool
    type Transaction: Transaction + Send + 'static;

    type OriginTransaction: OriginTransaction + Send + 'static;

    async fn check(&mut self, _index: usize, _tx: Self::Transaction) -> Result<CheckResponse> {
        Ok(Default::default())
    }

    async fn validate(
        &self,
        _index: usize,
        _tx: Self::OriginTransaction,
    ) -> Result<Self::Transaction> {
        Ok(Self::Transaction::default())
    }

    // From genesis.
    async fn genesis(&mut self, _index: usize)
    where
        Self::Context: ContextMut,
    {
    }

    // From block.
    async fn apply_txs(&mut self, _index: usize, _tx: &[Self::Transaction]) -> ExecResults
    where
        Self::Context: ContextMut,
    {
        Default::default()
    }
}
