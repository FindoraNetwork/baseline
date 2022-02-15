#[derive(Debug)]
pub enum Error {
    NoStartSupport,
}

pub type Result<T> = core::result::Result<T, Error>;

