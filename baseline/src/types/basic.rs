use alloc::vec::Vec;

#[derive(Debug, Clone, Default)]
pub struct BlockHash(pub Vec<u8>);

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHeight(pub i64);

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct TransactionHash(pub Vec<u8>);

impl From<Vec<u8>> for TransactionHash {
    fn from(e: Vec<u8>) -> Self {
        Self (e)
    }
}

#[derive(Debug, Clone, Default)]
pub struct NodeAddress(pub Vec<u8>);

#[derive(Debug, Clone, Default)]
pub struct MerkleHash(pub Vec<u8>);

#[derive(Debug, Clone, Default)]
pub struct ModuleVersion(pub u64);

#[derive(Debug, Clone, Default)]
pub struct Power(pub u64);
