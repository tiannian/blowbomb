use std::fmt::Debug;

use crate::{Error, Result};

pub type Txid = FixedBytes<32>;

#[derive(PartialEq)]
pub struct LeafId {
    pub txid: Txid,
    pub index: u32,
}

impl Debug for LeafId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LeafId {{ {:?}:{} }}", self.txid, self.index)
    }
}

pub type IndexKey = FixedBytes<32>;

pub type Address = FixedBytes<20>;

#[derive(PartialEq)]
pub struct FixedBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> Debug for FixedBytes<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FixedBytes {{ {} }}", hex::encode(self.0))
    }
}

impl<const N: usize> FixedBytes<N> {
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != N {
            return Err(Error::WrongLengthForFixedBytes(slice.len(), N));
        }

        Ok(Self(slice.try_into().unwrap()))
    }
}

#[derive(PartialEq)]
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
