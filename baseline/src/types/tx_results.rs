use alloc::{vec::Vec, string::String};

#[derive(Debug, Default, Clone)]
pub struct ExecResult {
    pub data: Vec<u8>,
    pub log: String,
    pub info: String,
    pub gas_wanted: i64,
    pub gas_used: i64,
}

#[derive(Debug, Default, Clone)]
pub struct ExecResults {
    pub results: Vec<ExecResult>
}
