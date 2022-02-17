use async_trait::async_trait;
use baseline::types::ExecResults;

#[async_trait]
pub trait Consensus: Send + Sync {
    async fn init_chain(&self);

    async fn apply_block(&self, txs: Vec<Vec<u8>>) -> ExecResults;

    async fn rollback_block(&self) {}
}

pub trait ConsensusCtl<App: Consensus> {
    fn set_app(&mut self, c: App);
}
