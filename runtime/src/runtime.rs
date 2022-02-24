use std::{marker::PhantomData, sync::Arc};

use baseline::prelude::{ContextMut, Manager};

use crate::{AppRuntime, ConsensusCtl, Mempool, MempoolCtl};

pub struct Runtime<M, C, P> {
    consensus: Option<C>,
    mempool: Option<P>,
    marker: PhantomData<M>,
}

impl<M, C, P> Default for Runtime<M, C, P> {
    fn default() -> Self {
        Self {
            consensus: None,
            mempool: None,
            marker: PhantomData,
        }
    }
}

impl<M, C, P> Runtime<M, C, P>
where
    M: Manager,
    M::Context: ContextMut,
    C: ConsensusCtl<AppRuntime<M>>,
    P: MempoolCtl<AppRuntime<M>>,
{
    pub fn app(&mut self, app: M) {
        let app = AppRuntime::new(app.clone());

        let arc = Arc::new(app);

        if let Some(consensus) = &mut self.consensus {
            consensus.set_app(arc.clone());
        }

        if let Some(mempool) = &mut self.mempool {
            mempool.set_app(arc.clone())
        }
    }

    pub fn consensus(&mut self, c: C) {
        self.consensus = Some(c);
    }

    pub fn mempool(&mut self, m: P) {
        self.mempool = Some(m);
    }
}
