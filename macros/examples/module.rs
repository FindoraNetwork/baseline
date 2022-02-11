use baseline::{prelude::Context, Metadata};
use baseline_macros::module;

#[module]
pub struct MockModule<C: Context> {
    #[context]
    ctx: C,

    #[metadata(name = "mock", version = 0, impl_version = "0.1.0", target_height = 0)]
    metadata: Metadata,
}

// impl<C: Context> ModuleMetadata for MockModule<C> {
// fn metadata() -> Metadata {
//     Metadata {
//         name: String::from("mock"),
//         version: 0,
//         impl_version: String::from("0.1.1"),
//         target_height: 1,
//     }
// }
// }

fn main() {}
