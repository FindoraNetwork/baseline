use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use baseline::{
    digest::Digest,
    prelude::{Context, ContextMut, Manager},
    types::Block,
};

use crate::{typedef::RwLock, Consensus, Mempool, ModuleCheckResponse};

pub struct AppRuntime<M: Manager> {
    pub deliver_state: Arc<RwLock<M>>,

    pub check_state: Arc<RwLock<M>>,
}

impl<M> AppRuntime<M>
where
    M: Manager,
{
    pub fn new(m: M) -> Self {
        // let _modules = m.modules();

        let deliver_state = Arc::new(RwLock::new(m.clone()));
        let check_state = Arc::new(RwLock::new(m));

        // TODO: Parse metadata set.

        // log::info!("Build modules stat: {:#?} ", indexs);

        Self {
            deliver_state,

            check_state,
        }
    }

    fn sync(&self) {}
}

#[async_trait]
impl<M> Consensus for AppRuntime<M>
where
    M: Manager,
{
    async fn apply_block(
        &self,
        block: Block<Vec<u8>>,
    ) -> (baseline::types::ExecResults, baseline::types::Consensus) {
        {
            // validate txs first.
        }

        {
            // Set block.
            // let mut b = self.deliver_state.write().await;

            // let bb = b.ctx_mut().block_mut();

            // bb.push_block(block);
        }

        {
            // clear tx.
            // for tx in &block.txs {

            // }
        }

        // TODO: Set result back.

        // TODO: Sum merkle here.

        // Sync check to deliver.
        self.sync();

        Default::default()
    }
}

#[async_trait]
impl<M> Mempool for AppRuntime<M>
where
    M: Manager,
{
    async fn check(&self, tx: Vec<u8>) -> baseline::Result<ModuleCheckResponse> {
        let (tx, txid) = {
            let inner = self.check_state.read().await;

            let ptx = inner.validate(0, &tx).await?;

            let txid = <M::Context as Context>::Digest::digest(&tx).to_vec();

            let mut mp = self.check_state.write().await;

            let ctx = mp.ctx_mut();

            let mempool = ctx.mempool_mut();

            mempool.txs.insert(txid.clone().into(), ptx.clone());

            (ptx, txid)
        };

        // Call check by id and version.
        let data = {
            let state = self.check_state.read().await;

            let context = state.ctx();

            let height = &context.block().height;

            let pms = &context.governance().metadatas;

            let mut indexs = Vec::new();

            {
                let inner = self.check_state.read().await;

                for pm in pms {
                    let idx = pm.next_height(height).expect("internal error.");

                    let name = pm.name();

                    indexs.push((idx, name));
                }

                for (idx, _) in &indexs {
                    inner.check(*idx, &tx).await?;
                }
            }

            let mut ress = BTreeMap::new();

            {
                let mut inner = self.check_state.write().await;

                for (idx, name) in &indexs {
                    let res = inner.apply_check(*idx, &tx).await?;

                    ress.insert(name.to_string(), res.data);
                }
            }

            ress
        };

        Ok(ModuleCheckResponse { txid, data })
    }
}
