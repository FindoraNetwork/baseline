use baseline::{prelude::Context, Metadata, bs3::{Map, merkle::AppendOnlyMerkle}};
use baseline_macros::module;

#[module]
pub struct Mock2Module<C: Context> {
    #[context]
    ctx: C,

    #[metadata(name = "mock", version = 0, impl_version = "0.1.0", target_height = 0)]
    pub metadata: Metadata,
}

#[module]
pub struct MockModule<C: Context> {
    #[context]
    ctx: C,

    #[metadata(name = "mock", version = 0, impl_version = "0.1.0", target_height = 0)]
    pub metadata: Metadata,

    #[storage]
    pub value: Map<i64, i64>,

    #[storage(merkle = "AppendOnlyMerkle")]
    pub merkle_value: Map<i64, i64>,

    #[dependence]
    pub mock2: Mock2Module<C>,
}

// impl<C: Context> MockModule<C> {
    // fn new() -> Self {
    //     use baseline::prelude::ModuleMetadata;
    //
    //     let metadata = Self::metadata();
    //
    //     Self {
    //         metadata,
    //     }
    // }
// }

fn main() {}
