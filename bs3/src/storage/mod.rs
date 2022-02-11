use crate::{prelude::Model, backend::Backend};

pub struct Storage<V: Model, M, B: Backend> {
    pub value: V,
    pub merkle: M,
    pub backend: B,
}
