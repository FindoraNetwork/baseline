use baseline::prelude::Manager;

pub struct Runtime<M: Manager> {
    pub manager: M,
}

impl<M: Manager> Runtime<M> {
    pub fn start(self) {}

    pub fn step_block(&self, _txs: Vec<Vec<u8>>) {}

    pub fn rollback_block(&self) {}
}
