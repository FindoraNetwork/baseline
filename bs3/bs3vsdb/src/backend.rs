use bs3::{BranchName, prelude::{Backend, VersionedBackend, Forkable, Versionable}, Error, Result};
use vsdb::{MapxOrdVs, VsMgmt};


#[derive(Clone)]
pub struct VsDbBackend {
    branch_name: BranchName,
    vsdb: MapxOrdVs<Vec<u8>, Vec<u8>>,
}

impl Backend for VsDbBackend {}

impl VersionedBackend for VsDbBackend {}

impl Forkable for VsDbBackend {
    fn fork(&self, target: &[u8]) -> Result<Self> {
        let new = self.clone();

        let base_branch_name = vsdb::ParentBranchName(&self.branch_name.0);

        let branch_name = vsdb::BranchName(target);

        new.vsdb
            .branch_create_by_base_branch(branch_name, base_branch_name)
            .map_err(|e| Error::BackendError(format!("{}", e)))?;

        Ok(new)
    }

    fn merge(&self, _branch: Self) -> Result<()> {
        let branch_name = vsdb::BranchName(&self.branch_name.0);

        self.vsdb
            .branch_merge_to_parent(branch_name)
            .map_err(|e| Error::BackendError(format!("{}", e)))?;

        Ok(())
    }
}

impl Versionable for VsDbBackend {
    fn commit(&self, name: &[u8]) -> Result<()> {
        let version_name = vsdb::VersionName(name);
        let branch_name = vsdb::BranchName(&self.branch_name.0);

        self.vsdb
            .version_create_by_branch(version_name, branch_name)
            .map_err(|e| Error::BackendError(format!("{}", e)))?;
        Ok(())
    }

    fn pop(&self) -> Result<()> {
        self.vsdb
            .version_pop()
            .map_err(|e| Error::BackendError(format!("{}", e)))?;
        Ok(())
    }
}
