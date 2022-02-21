use std::marker::PhantomData;

use baseline::prelude::{ContextMut, Manager};

use crate::{ConsensusCtl, ConsensusRuntime};

pub struct Runtime<M, C> {
    pub consensus: Option<C>,
    pub marker: PhantomData<M>,
}

impl<M, C> Runtime<M, C>
where
    M: Manager,
    M::Context: ContextMut,
    C: ConsensusCtl<ConsensusRuntime<M>>,
{
    pub fn app(&mut self, app: M) {
        if let Some(consensus) = &mut self.consensus {
            consensus.set_app(ConsensusRuntime::new(app.clone()));
        }
    }

    pub fn consensus(&mut self, c: C) {
        self.consensus = Some(c);
    }

    pub fn start(self) {}
}
