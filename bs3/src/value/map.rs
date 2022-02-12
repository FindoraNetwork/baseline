use alloc::collections::BTreeMap;

use crate::Storage;

#[derive(Clone)]
pub struct Map<K, V> {
    inner: BTreeMap<K, V>,
}

// impl<K, V> Model for Map<K, V>
// where
//     K: KeyEnDe,
//     V: ValueEnDe,
// {
//
// }

impl<B, M, K, V> Storage<Map<K, V>, M, B> {}
