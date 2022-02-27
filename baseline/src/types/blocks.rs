use alloc::string::String;

use super::{BlockHash, BlockHeight, MerkleHash, NodeAddress, Timestamp};

#[derive(Debug, Clone, Default)]
pub struct BlockHeader {
    pub chain_id: String,
    pub height: BlockHeight,
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
    pub height: BlockHeight,
    pub hash: BlockHash,
    // TODO: Runtime.
}

impl Blocks {
    pub fn latest(&self) -> BlockHeight {
        Default::default()
    }

    pub fn get_block_by_height(&self, _height: BlockHeight) -> Block {
        Default::default()
    }

    pub fn push_block(&mut self, block: Block) {
        self.height = block.headers.height;
        self.hash = block.hash;

        // TODO: insert into block store.
    }
}
