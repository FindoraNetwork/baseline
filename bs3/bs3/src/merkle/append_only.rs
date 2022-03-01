use core::marker::PhantomData;

use digest::Digest;

use crate::prelude::Merkle;

pub struct AppendOnlyMerkle<D> {
    marker: PhantomData<D>,
}

impl<D> Clone for AppendOnlyMerkle<D> {
    fn clone(&self) -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<D: Digest + Send> Merkle for AppendOnlyMerkle<D> {
    type Digest = D;

    fn new(_digest: D) -> Self {
        Self {
            marker: PhantomData,
        }
    }
}
