use digest::Digest;

pub trait Merkle: Clone {
    type Digest: Digest + Send;

    fn new(digest: Self::Digest) -> Self;
}
