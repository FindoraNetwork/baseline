use async_trait::async_trait;

use crate::{error::Result, types::CheckResponse};

use super::{Module, OriginTransaction, Transaction};

use alloc::boxed::Box;

#[async_trait]
pub trait Mempool: Module {
    type Transaction: Transaction + Send + 'static;

    type OriginTransaction: OriginTransaction + Send + 'static;

    async fn check(&self, _tx: Self::Transaction) -> Result<CheckResponse> {
        Ok(Default::default())
    }

    async fn validate(&self, _tx: Self::OriginTransaction) -> Result<Self::Transaction> {
        Ok(Self::Transaction::default())
    }
}
