use baseline::prelude::Manager;

use crate::ConsensusCtl;

pub struct Runtime<M: Manager, C: ConsensusCtl> {
    pub manager: M,
    pub consensus: C,
}

impl<M: Manager, C: ConsensusCtl> Runtime<M, C> {
    pub fn start(self) {}

    pub fn step_block(&self, _txs: Vec<Vec<u8>>) {}

    pub fn rollback_block(&self) {}
}
