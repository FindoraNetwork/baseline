use baseline_runtime::Consensus;

use crate::Error;

pub struct ConsensusRuntime<C: Consensus> {
    rt: C,
}

// impl<C: Consensus> ConsensusCtl for ConsensusRuntime<C> {
// type App = C;
//
// fn new(rt: C) -> Self {
//     Self { rt }
// }
// }

impl<C: Consensus> ConsensusRuntime<C> {
    pub fn start(&self) -> Result<(), Error> {
        // No impl

        Err(Error::NoStartSupport)
    }

    pub async fn step_block(&self, txs: Vec<Vec<u8>>) -> Result<(), Error> {
        self.rt.apply_block(txs).await;
        Ok(())
    }

    pub async fn rollback_block(&self) -> Result<(), Error> {
        self.rt.rollback_block().await;
        Ok(())
    }
}
