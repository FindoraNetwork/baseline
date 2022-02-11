use crate::{prelude::Module, Metadata};

pub trait ModuleMetadata: Module {
    fn metadata() -> Metadata;
}
