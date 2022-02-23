use std::sync::Arc;

use async_trait::async_trait;
use baseline::{types::CheckResponse, Result};

#[async_trait]
pub trait Mempool {
    async fn check(&self, _tx: Vec<u8>) -> Result<CheckResponse> {
        Ok(Default::default())
    }
}

pub trait MempoolCtl<App: Mempool> {
    fn set_app(&mut self, c: Arc<App>);
}
