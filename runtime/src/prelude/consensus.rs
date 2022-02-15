use async_trait::async_trait;
use baseline::types::ExecResults;

#[async_trait]
pub trait Consensus {
    async fn init_chain(&self);

    async fn apply_block(&self, txs: Vec<Vec<u8>>) -> ExecResults;

    fn rollback_block(&self) {}
}

pub trait ConsensusCtl {
    type App: Consensus;

    fn new(c: Self::App) -> Self;
}

