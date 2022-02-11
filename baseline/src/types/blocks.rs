use alloc::string::String;

use super::{BlockHash, BlockHeight, MerkleHash, NodeAddress, Timestamp};

#[derive(Debug, Clone, Default)]
pub struct BlockHeader {
    pub chain_id: String,
    pub height: i64,
    pub time: Option<Timestamp>,
    pub txs_hash: MerkleHash,
    pub app_hash: MerkleHash,
    pub proposer_address: NodeAddress,
}

#[derive(Debug, Clone, Default)]
pub struct Block {
    pub hash: BlockHash,
    pub headers: BlockHeader,
}

#[derive(Debug, Clone, Default)]
pub struct Blocks {
    // TODO: Runtime.
}

impl Blocks {
    pub fn latest(&self) -> BlockHeight {
        Default::default()
    }

    pub fn height(&self, _height: BlockHeight) -> Block {
        Default::default()
    }
}
