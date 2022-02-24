use std::sync::Arc;

use baseline::{prelude::Manager, types::Block};
use baseline_runtime::{AppRuntime, Consensus, ConsensusCtl};

use crate::Error;

pub struct ConsensusRuntime<M: Manager> {
    rt: Arc<AppRuntime<M>>,
}

impl<M: Manager> ConsensusCtl<AppRuntime<M>> for ConsensusRuntime<M> {
    fn set_app(&mut self, c: Arc<AppRuntime<M>>) {
        self.rt = c;
    }
}

impl<M: Manager> ConsensusRuntime<M> {
    pub fn start(&self) -> Result<(), Error> {
        Err(Error::NoStartSupport)
    }

    pub async fn step_block(&self, block: Block, txs: Vec<Vec<u8>>) -> Result<(), Error> {

        self.rt.apply_block(block, txs).await;
        Ok(())
    }

    pub async fn rollback_block(&self) -> Result<(), Error> {
        self.rt.rollback_block().await;
        Ok(())
    }
}
