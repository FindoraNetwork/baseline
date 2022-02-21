use alloc::vec::Vec;

use crate::Result;

pub trait ValueEn {
    fn encode_value(&self) -> Vec<u8>;
}

/// Methods used to decode the VALUE.
pub trait ValueDe: Sized {
    fn decode_value(bytes: &[u8]) -> Result<Self>;
}

/// Methods used to encode and decode the VALUE.
pub trait ValueEnDe: ValueEn + ValueDe {
    /// Encode original key type to bytes.
    fn encode(&self) -> Vec<u8> {
        <Self as ValueEn>::encode_value(self)
    }

    /// Decode from bytes to the original key type.
    fn decode(bytes: &[u8]) -> Result<Self> {
        <Self as ValueDe>::decode_value(bytes)
    }
}

impl ValueEn for Vec<u8> {
    fn encode_value(&self) -> Vec<u8> {
        self.clone()
    }
}

impl ValueDe for Vec<u8> {
    fn decode_value(bytes: &[u8]) -> Result<Self> {
        Ok(bytes.to_vec())
    }
}

impl ValueEnDe for Vec<u8> {}
