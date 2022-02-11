#![no_std]
#![feature(generic_associated_types)]

extern crate alloc;

pub mod prelude;

pub mod types;

mod error;
pub use error::*;

pub mod context;

mod metadata;
pub use metadata::*;

mod rpc;
pub use rpc::*;

pub use bs3;

pub use baseline_macros::*;
