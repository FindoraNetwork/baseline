pub trait Transaction: Default + Sized + Sync + Send {}

pub trait OriginTransaction: Default + Sized + Sync + Send {}

impl Transaction for () {}

impl OriginTransaction for () {}
