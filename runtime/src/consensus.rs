use std::sync::Arc;

use async_trait::async_trait;
use baseline::{
    prelude::{ContextMut, Manager},
    types::ExecResults,
};
use futures_utils_lite::zip_array;

use crate::{
    typedef::{spawn, RwLock},
    Consensus,
};

pub struct ConsensusRuntime<M: Manager> {
    pub inner: Arc<RwLock<M>>,
}
impl<M> ConsensusRuntime<M>
where
    M: Manager,
{
    pub fn new(m: M) -> Self {
        let inner = Arc::new(RwLock::new(m));

        Self { inner }
    }
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
        // build context and set context.

        let mut tx_handles = Vec::new();

        let mut res = ExecResults::with_capacity(otxs.len());

        for otx in otxs {
            let inner = self.inner.clone();

            let handler = spawn(async move {
                let inn = inner.read().await;

                inn.validate(otx).await
            });

            tx_handles.push(handler);
        }

        let txs = zip_array(tx_handles).await;

        let mut validate_txs = Vec::new();

        for (tx, r) in txs.into_iter().zip(res.results.iter_mut()) {
            match tx {
                Ok(t) => {
                    validate_txs.push(t);
                }
                Err(e) => {
                    *r = Err(e);
                }
            }
        }

        let mut inner = self.inner.write().await;

        inner.apply_txs(&validate_txs).await;

        // TODO: Set result back.

        // TODO: Sum merkle here.

        Default::default()
    }
}
