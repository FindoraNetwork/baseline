use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::types::{BlockHeight, ModuleVersion};

#[derive(Clone, Debug, Default)]
pub struct Metadata {
    pub name: String,
    pub version: ModuleVersion,
    pub impl_version: String,
    pub target_height: BlockHeight,
}

#[derive(Clone, Debug, Default)]
pub struct ParsedMetadata {
    name: String,
    height_idx: BTreeMap<BlockHeight, usize>,
    version_idx: BTreeMap<ModuleVersion, usize>,
    idx: Vec<Metadata>,
}

impl ParsedMetadata {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn next_height(&self, height: &BlockHeight) -> Option<usize> {
        let idx_range = self.height_idx.range(..height).next_back();

        if let Some((_h, i)) = idx_range {
            Some(*i)
        } else {
            None
        }
    }

    pub fn modify_height(&mut self, version: &ModuleVersion, target_height: &BlockHeight) {
        if let Some(idx) = self.version_idx.get(version) {
            let metadata = &mut self.idx[*idx];

            let origin_height = &metadata.target_height;

            self.height_idx.remove(origin_height);
            self.height_idx.insert(target_height.clone(), *idx);

            metadata.target_height = target_height.clone();
        }
    }

    pub fn metadata(&self, version: &ModuleVersion) -> Option<&Metadata> {
        if let Some(idx) = self.version_idx.get(version) {
            Some(&self.idx[*idx])
        } else {
            None
        }
    }
}
