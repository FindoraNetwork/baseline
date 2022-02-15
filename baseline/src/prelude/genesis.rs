use async_trait::async_trait;

use super::{ContextMut, Module};

use alloc::boxed::Box;

#[async_trait]
pub trait Genesis: Module {
    async fn genesis(&mut self)
    where
        Self::Context: ContextMut,
    {
    }
}
