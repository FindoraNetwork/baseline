#![no_std]
#![feature(generic_associated_types)]

extern crate alloc;

pub mod prelude;

pub mod types;

mod error;
pub use error::*;

mod context;
pub use context::*;

mod metadata;
pub use metadata::*;

pub mod rpc;

// mod version;
// pub use version::*;

// Reexport crate.
pub use bs3;

pub use async_trait::async_trait;

pub use baseline_macros::*;
