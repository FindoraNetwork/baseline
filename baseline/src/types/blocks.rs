use core::marker::PhantomData;

use alloc::{string::String, vec::Vec};

use crate::prelude::Transaction;

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
pub struct Block<T: Transaction> {
    pub hash: BlockHash,
    pub headers: BlockHeader,
    pub txs: Vec<T>,
}

#[derive(Debug, Clone, Default)]
pub struct Blocks<T: Transaction> {
    pub height: BlockHeight,
    pub hash: BlockHash,
    pub marker: PhantomData<T>,
}

impl<T: Transaction> Blocks<T> {
    pub fn latest(&self) -> BlockHeight {
        Default::default()
    }

    pub fn get_block_by_height(&self, _height: BlockHeight) -> Block<T> {
        Default::default()
    }

    pub fn push_block(&mut self, block: Block<T>) {
        self.height = block.headers.height;
        self.hash = block.hash;

        // TODO: insert into block store.
    }
}
