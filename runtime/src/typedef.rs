#[cfg(feature = "smol_backend")]
mod smol_backend {
    pub type RwLock<T> = smol::lock::RwLock<T>;

    pub use smol::spawn;
}

#[cfg(feature = "smol_backend")]
pub use smol_backend::*;
