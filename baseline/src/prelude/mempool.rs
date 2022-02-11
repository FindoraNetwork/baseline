use async_trait::async_trait;

use crate::{error::MempoolResult, types::CheckResponse};

use super::{Transaction, OriginTransaction};

use alloc::boxed::Box;

#[async_trait]
pub trait Mempool {
    type Transaction: Transaction;

    type OriginTransaction: OriginTransaction;

    async fn check(&mut self, _tx: Self::Transaction) -> MempoolResult<CheckResponse> {
        Ok(Default::default())
    }

    async fn validate(&mut self, _tx: Self::OriginTransaction) -> MempoolResult<Self::Transaction> {
        Ok(Self::Transaction::default())
    }
}
