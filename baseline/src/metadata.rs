use alloc::string::String;

use crate::types::{BlockHeight, ModuleVersion};

#[derive(Clone, Debug)]
pub struct Metadata {
    pub name: String,
    pub version: ModuleVersion,
    pub impl_version: String,
    pub target_height: BlockHeight,
}
