use core::marker::PhantomData;

use digest::Digest;

use crate::prelude::Merkle;

pub struct EmptyMerkle<D> {
    marker: PhantomData<D>,
}

impl<D: Digest> Clone for EmptyMerkle<D> {
    fn clone(&self) -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<D: Digest + Sync + Send> Merkle for EmptyMerkle<D> {
    type Digest = D;

    fn new(_digest: D) -> Self {
        Self {
            marker: PhantomData,
        }
    }
}
