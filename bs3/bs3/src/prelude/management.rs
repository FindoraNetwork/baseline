// use crate::BranchName;

use crate::Result;

pub trait Forkable: Sized {
    fn fork(&self, target: &[u8]) -> Result<Self>;

    fn merge(&self, branch: Self) -> Result<()>;
}

pub trait Versionable {
    fn commit(&self, name: &[u8]) -> Result<()>;

    fn pop(&self) -> Result<()>;
}
