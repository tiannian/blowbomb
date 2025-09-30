use crate::{Error, Result};

pub type Txid = FixedBytes<32>;

pub struct LeafId {
    pub txid: Txid,
    pub index: u32,
}

pub type IndexKey = FixedBytes<32>;

pub type Address = FixedBytes<20>;

pub struct FixedBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> FixedBytes<N> {
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != N {
            return Err(Error::WrongLengthForFixedBytes(slice.len(), N));
        }

        Ok(Self(slice.try_into().unwrap()))
    }
}

pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }
}
