use std::sync::Arc;

use baseline::types::Block;
use baseline_runtime::{Consensus, ConsensusCtl};

use crate::Error;

pub struct ConsensusRuntime<C: Consensus> {
    rt: Arc<C>,
}

impl<C: Consensus> ConsensusCtl<C> for ConsensusRuntime<C> {
    fn set_app(&mut self, c: Arc<C>) {
        self.rt = c;
    }
}

impl<C: Consensus> ConsensusRuntime<C> {
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
