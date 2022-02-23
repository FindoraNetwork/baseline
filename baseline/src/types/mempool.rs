use alloc::collections::BTreeMap;

use crate::prelude::Transaction;

use super::TransactionHash;

#[derive(Clone, Default)]
pub struct Mempool<T: Transaction> {
    pub txs: BTreeMap<TransactionHash, T>,
}
