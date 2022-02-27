use alloc::vec::Vec;

use crate::ParsedMetadata;

#[derive(Clone, Debug, Default)]
pub struct Governance {
    pub metadatas: Vec<ParsedMetadata>,
}
