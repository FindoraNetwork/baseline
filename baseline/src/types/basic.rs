#[derive(Debug, Clone, Default)]
pub struct BlockHash([u8; 32]);

#[derive(Debug, Clone, Default)]
pub struct BlockHeight(i64);

#[derive(Debug, Clone, Default)]
pub struct TransactionHash([u8; 32]);

#[derive(Debug, Clone, Default)]
pub struct NodeAddress([u8; 20]);

#[derive(Debug, Clone, Default)]
pub struct MerkleHash([u8; 32]);
