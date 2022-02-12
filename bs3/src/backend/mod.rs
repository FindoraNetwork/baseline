use crate::prelude::{Forkable, Versionable};

pub trait Backend: Sized + Clone + VersionedBackend {}

/// If you backend already support versioned function.
pub trait VersionedBackend: Forkable + Versionable {}

#[cfg(feature = "vsdb-backend")]
pub mod vsdb;
