use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::{
    prelude::{self, AsyncRuntime, Transaction},
    types::{self, Blocks, Consensus, Event, Governance, Mempool},
};

pub struct Context<B, D, R, T>
where
    T: Transaction,
{
    pub backend: B,
    marker_d: PhantomData<D>,
    async_runtime: R,

    pub events: Vec<Event>,
    pub consensus: Consensus,
    pub governance: Governance,
    pub block: Blocks<T>,
    pub mempool: Mempool<T>,
}

impl<B, D, R, T> Clone for Context<B, D, R, T>
where
    B: bs3::prelude::Backend + Send + Sync + 'static,
    D: digest::Digest + Send + Sync + 'static,
    R: AsyncRuntime,
    T: Transaction,
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

impl<B, D, R, T> prelude::Context for Context<B, D, R, T>
where
    B: bs3::prelude::Backend + Send + Sync + 'static,
    D: digest::Digest + Send + Sync + 'static,
    R: AsyncRuntime + Send + Sync + 'static,
    T: Transaction,
{
    type Store = B;

    type Digest = D;

    type Task<Ret> = R::Task<Ret>;

    type Transaction = T;

    fn emmit(&mut self, event: impl prelude::Event) {
        self.events.push(event.to_event())
    }

    fn consensus(&self) -> &types::Consensus {
        &self.consensus
    }

    fn governance(&self) -> &types::Governance {
        &self.governance
    }

    fn spwan<Ret: Send + 'static>(
        &self,
        handler: impl core::future::Future<Output = Ret> + Send + 'static,
    ) -> Self::Task<Ret> {
        self.async_runtime.spwan(handler)
    }

    fn block(&self) -> &types::Blocks<T> {
        &self.block
    }

    fn mempool(&self) -> &types::Mempool<T> {
        &self.mempool
    }
}

impl<B, D, R, T> prelude::ContextMut for Context<B, D, R, T>
where
    B: bs3::prelude::Backend + Send + Sync + 'static,
    D: digest::Digest + Send + Sync + 'static,
    R: AsyncRuntime + Send + Sync + 'static,
    T: Transaction,
{
    fn consensus_mut(&mut self) -> &mut types::Consensus {
        &mut self.consensus
    }

    fn governance_mut(&mut self) -> &mut types::Governance {
        &mut self.governance
    }

    fn block_mut(&mut self) -> &mut types::Blocks<T> {
        &mut self.block
    }

    fn mempool_mut(&mut self) -> &mut types::Mempool<Self::Transaction> {
        &mut self.mempool
    }
}

impl<B, D, R, T> prelude::ContextSetable for Context<B, D, R, T>
where
    B: bs3::prelude::Backend + Send + Sync + 'static,
    D: digest::Digest + Send + Sync + 'static,
    R: AsyncRuntime + Send + Sync + 'static,
    T: Transaction,
{
    fn store(&self) -> Self::Store {
        self.backend.clone()
    }

    fn digest(&self) -> Self::Digest {
        D::new()
    }

    fn new(backend: Self::Store) -> Self {
        Self {
            backend,
            marker_d: PhantomData,
            events: Vec::new(),
            consensus: Default::default(),
            block: Default::default(),
            mempool: Default::default(),
            governance: Default::default(),
            async_runtime: Default::default(),
        }
    }
}
