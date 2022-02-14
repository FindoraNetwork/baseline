use baseline::types::ExecResults;

pub trait Consensus {
    fn init_chain(&self);

    fn apply_block(&self, txs: Vec<Vec<u8>>) -> ExecResults;
}
