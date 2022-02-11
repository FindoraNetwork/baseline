use core::marker::PhantomData;

use crate::prelude::Model;

pub struct Map<K, V> {
    marker_k: PhantomData<K>,
    marker_v: PhantomData<V>,
}

impl<K, V> Model for Map<K, V> {}
