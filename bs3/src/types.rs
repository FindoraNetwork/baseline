use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct VersionName(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct BranchName(pub Vec<u8>);

impl From<&'_ str> for BranchName {
    fn from(e: &'_ str) -> Self {
        Self(e.as_bytes().to_vec())
    }
}
