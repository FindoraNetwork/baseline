#[cfg(feature = "smol-backend")]
mod smol_backend {
    pub type RwLock<T> = smol::lock::RwLock<T>;

    use baseline::prelude::AsyncRuntime;

    pub use smol::spawn;

    #[derive(Clone, Default)]
    pub struct SmolAsyncRuntime {}

    impl AsyncRuntime for SmolAsyncRuntime {
        type Task<T> = smol::Task<T>;

        fn spwan<R: Send + 'static>(
            &self,
            handler: impl core::future::Future<Output = R> + Send + 'static,
        ) -> Self::Task<R> {
            smol::spawn(handler)
        }
    }
}

#[cfg(feature = "smol-backend")]
pub use smol_backend::*;
