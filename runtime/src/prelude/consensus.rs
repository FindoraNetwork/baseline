use std::sync::Arc;

use async_trait::async_trait;
use baseline::types::{Block, ExecResults};

#[async_trait]
pub trait Consensus: Send + Sync {
    async fn apply_block(&self, block: Block, txs: Vec<Vec<u8>>) -> ExecResults;

    async fn rollback_block(&self) {}
}

pub trait ConsensusCtl<App: Consensus> {
    fn set_app(&mut self, c: Arc<App>);
}
