use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use baseline::Result;

#[derive(Debug, Clone, Default)]
pub struct ModuleCheckResponse {
    pub txid: Vec<u8>,
    pub data: BTreeMap<String, Vec<u8>>,
}

#[async_trait]
pub trait Mempool {
    async fn check(&self, _tx: Vec<u8>) -> Result<ModuleCheckResponse> {
        Ok(Default::default())
    }
}

pub trait MempoolCtl<App: Mempool> {
    fn set_app(&mut self, c: Arc<App>);
}
