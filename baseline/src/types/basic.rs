use alloc::vec::Vec;

#[derive(Debug, Clone, Default)]
pub struct BlockHash(pub Vec<u8>);

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlockHeight(pub i64);

#[derive(Debug, Clone, Default)]
pub struct TransactionHash(pub Vec<u8>);

#[derive(Debug, Clone, Default)]
pub struct NodeAddress(pub Vec<u8>);

#[derive(Debug, Clone, Default)]
pub struct MerkleHash(pub Vec<u8>);

#[derive(Debug, Clone, Default)]
pub struct ModuleVersion(pub u64);

#[derive(Debug, Clone, Default)]
pub struct Power(pub u64);
