use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use baseline::{
    prelude::{Context, ContextMut, ContextSetable, Manager},
    types::{Block, BlockHeight, CheckResponse},
    digest::Digest,
};

use crate::{typedef::RwLock, Consensus, Mempool};

pub struct AppRuntime<M: Manager> {
    pub inner: Arc<RwLock<M>>,
    pub indexs: Vec<BTreeMap<BlockHeight, usize>>,
    // Current block.
    pub context: RwLock<M::Context>,
}
impl<M> AppRuntime<M>
where
    M: Manager,
    M::Context: ContextSetable,
{
    pub fn new(m: M, backend: <M::Context as Context>::Store) -> Self {
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

        let context = RwLock::new(<M::Context>::new(backend));

        Self {
            inner,
            indexs,
            context,
        }
    }
}

#[async_trait]
impl<M> Consensus for AppRuntime<M>
where
    M: Manager,
    M::Context: ContextMut,
{
    async fn apply_block(&self, block: Block, _otxs: Vec<Vec<u8>>) -> baseline::types::ExecResults {
        {
            // Set block.
            let mut b = self.context.write().await;

            let bb = b.block_mut();

            bb.push_block(block);
        }

        let context = {
            let ctx = self.context.read().await;

            ctx.clone()
        };

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
impl<M> Mempool for AppRuntime<M>
where
    M: Manager,
{
    async fn check(&self, tx: Vec<u8>) -> baseline::Result<CheckResponse> {

        {
            let mut mp = self.context.write().await;

            let digest = <M::Context as Context>::Digest::digest(tx).to_vec();

            let mempool = mp.mempool_mut();

            let mut inner = self.inner.read().await;

            // got inner tx.
            // let ptx = inner.validate(0, tx).await?;

            mempool.txs.insert(digest.into(), Default::default());
        }

        // insert into mempool

        Ok(Default::default())
    }
}
