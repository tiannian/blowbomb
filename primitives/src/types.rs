use std::fmt::Debug;

use crate::{Error, Result};

pub type Txid = FixedBytes<32>;

pub type LeafId = FixedBytes<32>;

pub type IndexKey = FixedBytes<32>;

pub type Address = FixedBytes<20>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> FixedBytes<N> {
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != N {
            return Err(Error::WrongLengthForFixedBytes(slice.len(), N));
        }

        Ok(Self(slice.try_into().unwrap()))
    }

    pub fn is_none(&self) -> bool {
        self.0 == [0u8; N]
    }
}

impl<const N: usize> Default for FixedBytes<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> Debug for FixedBytes<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FixedBytes {{ {} }}", hex::encode(self.0))
    }
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);

impl Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bytes {{ {} }}", hex::encode(&self.0))
    }
}

impl Bytes {
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }
}
