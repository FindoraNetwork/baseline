use alloc::{string::String, vec::Vec};

use super::{BlockHash, TransactionHash};

#[derive(Clone)]
pub struct EventAttributes {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub index: bool,
}

#[derive(Clone)]
pub struct Event {
    pub ty: String,
    pub attributes: Vec<EventAttributes>,
    pub emmiter: Emmiter,
}

#[derive(Clone)]
pub enum Emmiter {
    Block(BlockHash),
    Transaction(TransactionHash),
}
