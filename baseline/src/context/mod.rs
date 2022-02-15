use core::marker::PhantomData;

use crate::{prelude, types};

pub struct Context<B, D, R> {
    pub backend: B,
    marker_d: PhantomData<D>,
    runtime: R,
}

impl<B, D, R> prelude::Context for Context<B, D, R>
where
    B: bs3::backend::Backend + Send + Sync,
    D: digest::Digest + Send + Sync,
    R: prelude::Runtime + Send + Sync,
{
    type Store = B;

    type Digest = D;

    type Task<T> = R::Task<T>;

    fn emmit(&mut self, event: impl prelude::Event) {
        self.runtime.emmit(event)
    }

    fn consensus(&self) -> &types::Consensus {
        self.runtime.consensus()
    }

    fn governance(&self) -> &types::Governance {
        self.runtime.governance()
    }

    fn spwan<Ret>(&mut self, handler: impl core::future::Future<Output = Ret>) -> Self::Task<Ret> {
        self.runtime.spwan(handler)
    }

    fn block(&self) -> &types::Blocks {
        self.runtime.block()
    }

    fn mempool(&self) -> &types::Mempool {
        self.runtime.mempool()
    }
}
