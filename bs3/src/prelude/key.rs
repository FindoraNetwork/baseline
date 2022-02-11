use alloc::vec::Vec;

use crate::Result;

pub trait KeyEn {
    fn encode_key(&self) -> Vec<u8>;
}

/// Methods used to decode the KEY.
pub trait KeyDe: Sized {
    fn decode_key(bytes: &[u8]) -> Result<Self>;
}

pub trait KeyEnDe: KeyEn + KeyDe {
    /// Encode original key type to bytes.
    fn encode(&self) -> Vec<u8> {
        <Self as KeyEn>::encode_key(self)
    }

    /// Decode from bytes to the original key type.
    fn decode(bytes: &[u8]) -> Result<Self> {
        <Self as KeyDe>::decode_key(bytes)
    }
}
