use alloc::vec::Vec;

use crate::Result;

#[derive(Debug, Default, Clone)]
pub struct ExecResult {
    pub data: Vec<u8>,
    pub gas_wanted: i64,
    pub gas_used: i64,
}

#[derive(Debug, Default, Clone)]
pub struct ExecResults {
    pub results: Vec<Result<ExecResult>>,
}

impl ExecResults {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut results = Vec::with_capacity(capacity);
        results.resize(capacity, Ok(ExecResult::default()));

        Self { results }
    }
}
