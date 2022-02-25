use alloc::vec::Vec;

use crate::Metadata;

#[derive(Clone, Default)]
pub struct Governance {
    pub metadatas: Vec<Metadata>,
}
