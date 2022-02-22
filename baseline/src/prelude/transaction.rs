use alloc::vec::Vec;

pub trait Transaction: Default + Sync + Send {}

pub trait OriginTransaction: Default + Sync + Send {}

impl Transaction for () {}

impl Transaction for Vec<u8> {}

impl OriginTransaction for () {}

impl OriginTransaction for Vec<u8> {}
