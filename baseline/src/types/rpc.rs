use alloc::{string::String, vec::Vec};

use crate::RpcError;

pub struct Request {
    pub method: String,
    pub params: Vec<u8>,
}

pub struct Response {
    pub code: i64,
    pub message: String,
    pub data: Vec<u8>,
}

impl Response {
    pub fn new(data: Vec<u8>) -> Response {
        Response {
            code: 0,
            message: String::from("success"),
            data,
        }
    }

    pub fn not_found() -> Response {
        let e = RpcError::not_found();

        e.to_response()
    }
}
