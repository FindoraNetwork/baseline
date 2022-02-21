use crate::types;

use super::Event;

pub trait Context: Send + Sync + 'static + Clone {
    type Store: bs3::backend::Backend + Send + Sync;

    type Digest: digest::Digest + Send + Sync;

    type Task<T>: core::future::Future<Output = T>;

    // Trigger event;
    fn emmit(&mut self, event: impl Event);
    // Spwan a new async work.
    fn spwan<R: Send + 'static>(
        &self,
        handler: impl core::future::Future<Output = R> + Send + 'static,
    ) -> Self::Task<R>;
    // Get block info.
    fn block(&self) -> &types::Blocks;
    // Get Consensus info.
    fn consensus(&self) -> &types::Consensus;
    // Get Governance info.
    fn governance(&self) -> &types::Governance;

    fn mempool(&self) -> &types::Mempool;
}

pub trait ContextMut: Context {
    fn consensus_mut(&mut self) -> &mut types::Consensus;

    fn governance_mut(&mut self) -> &mut types::Governance;
}

pub trait ContextSetable: ContextMut {
    fn store(&self) -> Self::Store;

    fn digest(&self) -> Self::Digest;
}
