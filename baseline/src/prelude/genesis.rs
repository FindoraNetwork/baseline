use super::{ContextMut, Module};

pub trait Genesis: Module {
    fn genesis(&mut self)
    where
        Self::Context: ContextMut,
    {
    }
}
