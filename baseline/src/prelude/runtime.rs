pub trait AsyncRuntime: Clone {
    type Task<T>: core::future::Future<Output = T>;

    // Spwan a new async work.
    fn spwan<R>(&self, handler: impl core::future::Future<Output = R>) -> Self::Task<R>;
}
