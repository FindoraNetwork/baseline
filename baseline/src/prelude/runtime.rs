pub trait AsyncRuntime: Clone {
    type Task<T>: core::future::Future<Output = T>;

    // Spwan a new async work.
    fn spwan<R: Send + 'static>(
        &self,
        handler: impl core::future::Future<Output = R> + Send + 'static,
    ) -> Self::Task<R>;
}
