use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::{
    prelude::{self, AsyncRuntime},
    types::{self, Consensus, Event, Governance, Blocks, Mempool},
};

pub struct Context<B, D, R> {
    pub backend: B,
    marker_d: PhantomData<D>,
    async_runtime: R,
    // marker_r: PhantomData<R>,

    events: Vec<Event>,
    consensus: Consensus,
    governance: Governance,
    block: Blocks,
    mempool: Mempool,
}

impl<B, D, R> Clone for Context<B, D, R>
where
    B: bs3::backend::Backend + Send + Sync + 'static,
    D: digest::Digest + Send + Sync + 'static,
    R: AsyncRuntime,
{
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
            marker_d: PhantomData,
            async_runtime: self.async_runtime.clone(),

            events: self.events.clone(),
            consensus: self.consensus.clone(),
            governance: self.governance.clone(),
            block: self.block.clone(),
            mempool: self.mempool.clone(),
        }
    }
}

impl<B, D, R> prelude::Context for Context<B, D, R>
where
    B: bs3::backend::Backend + Send + Sync + 'static,
    D: digest::Digest + Send + Sync + 'static,
    R: AsyncRuntime + Send + Sync + 'static,
{
    type Store = B;

    type Digest = D;

    type Task<T> = R::Task<T>;

    fn emmit(&mut self, event: impl prelude::Event) {
        self.events.push(event.to_event())
    }

    fn consensus(&self) -> &types::Consensus {
        &self.consensus
    }

    fn governance(&self) -> &types::Governance {
        &self.governance
    }

    fn spwan<Ret>(&self, handler: impl core::future::Future<Output = Ret>) -> Self::Task<Ret> {
        self.async_runtime.spwan(handler)
    }

    fn block(&self) -> &types::Blocks {
        &self.block
    }

    fn mempool(&self) -> &types::Mempool {
        &self.mempool
    }
}
