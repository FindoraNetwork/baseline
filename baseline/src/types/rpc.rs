use alloc::{string::String, vec::Vec};

pub struct Request {
    pub methods: String,
    pub params: Vec<u8>,
}

pub struct Response {
    pub code: i64,
    pub message: String,
    pub data: Vec<u8>,
}
