use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use baseline::{
    prelude::{ContextMut, Manager},
    types::{Block, BlockHeight, ExecResults, CheckResponse},
};

use crate::{typedef::RwLock, Consensus, Mempool};

pub struct AppRuntime<M: Manager> {
    inner: Arc<RwLock<M>>,
    indexs: Vec<BTreeMap<BlockHeight, usize>>,
    // Current block.
    block: Block,
    mempool: baseline::types::Mempool<M::Transaction>,
}
impl<M> AppRuntime<M>
where
    M: Manager,
{
    pub fn new(m: M) -> Self {
        let modules = m.modules();

        let mut indexs = Vec::new();

        let mut name = String::new();

        for (idx, module) in modules.into_iter().enumerate() {
            if name != module.name {
                let mut map = BTreeMap::new();

                map.insert(module.target_height, idx);

                indexs.push(map);
            } else {
                let index = indexs.len() - 1;

                let map = &mut indexs[index];

                map.insert(module.target_height, idx);
            }
            name = module.name;
        }

        let inner = Arc::new(RwLock::new(m));

        log::info!("Build modules stat: {:#?} ", indexs);

        Self {
            inner,
            indexs,
            block: Default::default(),
            mempool: Default::default(),
        }
    }
}

#[async_trait]
impl<M> Consensus for AppRuntime<M>
where
    M: Manager,
    M::Context: ContextMut,
{
    async fn apply_block(
        &self,
        _block: Block,
        _otxs: Vec<Vec<u8>>,
    ) -> baseline::types::ExecResults {
        // build context and set context.

        //         let mut tx_handles = Vec::new();
        //
        // let mut res = ExecResults::with_capacity(otxs.len());
        //
        // for otx in otxs {
        //     let inner = self.inner.clone();
        //
        //     let handler = spawn(async move {
        //         let inn = inner.read().await;
        //
        //         // inn.validate(otx).await
        //     });
        //
        //     tx_handles.push(handler);
        // }
        //
        // let txs = zip_array(tx_handles).await;
        //
        // let mut validate_txs = Vec::new();
        //
        // for (tx, r) in txs.into_iter().zip(res.results.iter_mut()) {
        //     match tx {
        //         Ok(t) => {
        //             validate_txs.push(t);
        //         }
        //         Err(e) => {
        //             *r = Err(e);
        //         }
        //     }
        // }
        //
        // let mut inner = self.inner.write().await;
        //
        // inner.apply_txs(&validate_txs).await;

        // TODO: Set result back.

        // TODO: Sum merkle here.

        Default::default()
    }
}

#[async_trait]
impl<M> Mempool for AppRuntime<M> where M: Manager {
    async fn check(&self, _tx: Vec<u8>) -> baseline::Result<CheckResponse> {
        // insert into mempool

        Ok(Default::default())
    }
}
