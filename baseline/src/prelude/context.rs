use crate::types;

use super::Event;

pub trait Context: Sync + Send {
    type Store: bs3::backend::Backend;

    type Digest: digest::Digest;

    // Trigger event;
    fn emmit(&mut self, event: impl Event);
    // Spwan a new async work.
    fn spwan<R>(&mut self, handler: impl core::future::Future<Output = R>) -> R;
    // Get block info.
    fn block(&self) -> &types::Blocks;
    // Get Consensus info.
    fn consensus(&self) -> &types::Consensus;
    // Get Governance info.
    fn governance(&self) -> &types::Governance;
}

pub trait ContextMut: Context {
    fn consensus_mut(&mut self) -> &mut types::Consensus;

    fn governance_mut(&mut self) -> &mut types::Governance;
}
