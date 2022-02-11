use alloc::{vec::Vec, string::String};

pub struct Request {
    pub methods: String,
    pub params: Vec<u8>,
}

pub struct Response {
    pub data: Vec<u8>,
}

