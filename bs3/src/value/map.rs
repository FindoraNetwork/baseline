use alloc::collections::BTreeMap;

use crate::prelude::{Model, KeyEnDe, ValueEnDe};

// use crate::Storage;

#[derive(Clone, Default)]
pub struct Map<K, V> {
    pub inner: BTreeMap<K, V>,
}

impl<K, V> Model for Map<K, V>
where
    K: KeyEnDe + Clone + Default,
    V: ValueEnDe + Clone + Default,
{

}

// impl<B, M, K, V> Storage<Map<K, V>, M, B> {}
