use async_trait::async_trait;
use baseline::{
    bs3::{merkle::AppendOnlyMerkle, Map},
    prelude::{Block, Context, ContextMut, Mempool},
    types::ExecResults,
    BlockResult, Metadata, RpcResult,
};

#[baseline::module]
pub struct Mock2Module<C: Context> {
    #[context]
    ctx: C,

    #[metadata(name = "mock", version = 0, impl_version = "0.1.0", target_height = 0)]
    pub metadata: Metadata,
}

#[baseline::module]
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

impl<C: Context> Mempool for MockModule<C> {
    type Transaction = ();

    type OriginTransaction = ();
}

#[async_trait]
impl<C: Context> Block for MockModule<C> {
    async fn apply_txs(&mut self, _tx: &[Self::Transaction]) -> BlockResult<ExecResults>
    where
        Self::Context: ContextMut,
    {
        let _s = self.ctx.consensus_mut();

        Ok(Default::default())
    }
}

#[baseline::rpc]
impl<C: Context> MockModule<C> {
    #[rpc(name = "call")]
    pub async fn call_method(&mut self, _params: String) -> RpcResult<Vec<u8>> {
        Ok(vec![])
    }

    #[rpc]
    pub async fn chain_id(&mut self) -> RpcResult<Vec<u8>> {
        Ok(vec![])
    }
}

fn main() {}
