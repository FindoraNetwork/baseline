use crate::types;

use super::Event;

pub trait Runtime {
    type Task<T>;

    // Trigger event;
    fn emmit(&mut self, event: impl Event);
    // Spwan a new async work.
    fn spwan<R>(&mut self, handler: impl core::future::Future<Output = R>) -> Self::Task<R>;
    // Get block info.
    fn block(&self) -> &types::Blocks;
    // Get Consensus info.
    fn consensus(&self) -> &types::Consensus;
    // Get Governance info.
    fn governance(&self) -> &types::Governance;

    fn mempool(&self) -> &types::Mempool;

    fn consensus_mut(&mut self) -> &mut types::Consensus;

    fn governance_mut(&mut self) -> &mut types::Governance;
}
