use crate::{backend::Backend, prelude::Model};

pub struct Storage<V: Model, M, B: Backend> {
    pub value: V,
    pub merkle: M,
    pub backend: B,
}
