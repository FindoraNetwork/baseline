use alloc::collections::BTreeMap;

use super::{NodeAddress, Power};

#[derive(Clone, Debug, Default)]
pub struct Consensus {
    pub validator_set: BTreeMap<NodeAddress, Power>,
    pub block_params: BlockParams,
}

#[derive(Clone, Default, Debug)]
pub struct BlockParams {
    pub max_bytes: i64,
    pub max_gas: i64,
}
