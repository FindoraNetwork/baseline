use alloc::vec::Vec;

pub trait Transaction: Default + Sized + Sync + Send {}

pub trait OriginTransaction: Default + Sized + Sync + Send {}

impl Transaction for () {}

impl Transaction for Vec<u8> {}

impl OriginTransaction for () {}

impl OriginTransaction for Vec<u8> {}
