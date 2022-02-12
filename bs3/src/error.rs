use alloc::string::String;

#[derive(Debug)]
pub enum Error {
    BackendError(String),
}

pub type Result<T> = core::result::Result<T, Error>;
