use alloc::{boxed::Box, vec::Vec};

use crate::{
    types::{CheckResponse, ExecResults},
    Metadata, Result,
};

use super::{Context, ContextMut, ContextSetable};

#[async_trait::async_trait]
pub trait Manager: Send + Sync + Clone {
    // From Context
    type Context: Context + ContextMut + ContextSetable;

    fn ctx(&self) -> &Self::Context;

    fn ctx_mut(&mut self) -> &mut Self::Context;

    // Module's metadata.
    fn modules(&self) -> Vec<Metadata>;

    async fn check(
        &self,
        _index: usize,
        _tx: &<Self::Context as Context>::Transaction,
    ) -> Result<()> {
        Ok(())
    }

    async fn apply_check(
        &mut self,
        _index: usize,
        _tx: &<Self::Context as Context>::Transaction,
    ) -> Result<CheckResponse> {
        Ok(Default::default())
    }

    async fn validate(
        &self,
        _index: usize,
        _tx: &[u8],
    ) -> Result<<Self::Context as Context>::Transaction> {
        Ok(Default::default())
    }

    // From genesis.
    async fn genesis(&mut self, _index: usize) {}

    // From block.
    async fn apply_txs(
        &mut self,
        _index: usize,
        _tx: &[<Self::Context as Context>::Transaction],
    ) -> ExecResults
    where
        Self::Context: ContextMut,
    {
        Default::default()
    }
}
