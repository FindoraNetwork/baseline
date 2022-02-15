use std::sync::Arc;

use async_trait::async_trait;
use baseline::{prelude::{Manager, ContextMut, Mempool}, types::ExecResults};
use futures_utils_lite::zip_array;

use crate::{Consensus, typedef::{RwLock, spawn}};

pub struct ConsensusRuntime<M: Manager> {
    pub inner: Arc<RwLock<M>>,
}

#[async_trait]
impl<M> Consensus for ConsensusRuntime<M>
where
    M: Manager,
    M::Context: ContextMut,
{
    async fn init_chain(&self) {
        let mut inner = self.inner.write().await;

        inner.genesis().await;
    }

    async fn apply_block(&self, otxs: Vec<Vec<u8>>) -> baseline::types::ExecResults {

        let tx_handles = Vec::new();

        for otx in otxs {
            let inner = self.inner.clone();

            let handler = spawn(async move {
                let mut inn = inner.write().await;

                inn.validate(otx).await
            });

            tx_handles.push(handler);
        }

        let txs = zip_array(tx_handles).await;

        let validate_txs = Vec::new();

//         for tx in txs {
        //     match tx {
        //         Ok(t) => {txs},
        //         Err(e) => {}
        //     }
        // }
//
        let mut inner = self.inner.write().await;

        inner.apply_txs(&txs).await
    }
}

