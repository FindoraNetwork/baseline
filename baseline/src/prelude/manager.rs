use alloc::vec::Vec;

use super::{Block, Mempool, RPC};

pub trait Manager: RPC + Mempool<OriginTransaction = Vec<u8>> + Block {}
