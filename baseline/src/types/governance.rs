use alloc::vec::Vec;

use crate::Metadata;

#[derive(Clone)]
pub struct Governance {
    pub metadatas: Vec<Metadata>,
}
