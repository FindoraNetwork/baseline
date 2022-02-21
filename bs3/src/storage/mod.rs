use crate::{
    backend::Backend,
    prelude::{Forkable, Merkle, Model, Versionable},
    BranchName, Result,
};

#[derive(Clone)]
pub struct Storage<V, M, B> {
    pub branch_name: BranchName,
    pub value: V,
    pub merkle: M,
    pub backend: B,
}

impl<V, M, B> Storage<V, M, B>
where
    V: Model,
    B: Backend,
    M: Merkle,
{
    pub fn new(backend: B, merkle: M) -> Self {
        Self {
            branch_name: BranchName::from("main"),
            value: V::default(),
            merkle,
            backend,
        }
    }
}

// TODO: Empty impl for more backend.
impl<V, M, B> Forkable for Storage<V, M, B>
where
    V: Model,
    B: Backend,
    M: Merkle,
{
    fn fork(&self, target: &[u8]) -> Result<Self> {
        let backend = self.backend.clone();

        backend.fork(target)?;

        Ok(Self {
            branch_name: BranchName(target.to_vec()),
            value: self.value.clone(),
            merkle: self.merkle.clone(),
            backend,
        })
    }

    fn merge(&self, branch: Self) -> Result<()> {
        self.backend.merge(branch.backend)?;
        Ok(())
    }
}

impl<V, M, B> Versionable for Storage<V, M, B>
where
    V: Model,
    B: Backend,
    M: Merkle,
{
    fn commit(&self, name: &[u8]) -> Result<()> {
        self.backend.commit(name)
    }

    fn pop(&self) -> Result<()> {
        self.backend.pop()
    }
}
