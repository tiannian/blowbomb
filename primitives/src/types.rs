use crate::{Error, Result};

pub type Txid = FixedBytes<32>;

pub struct LeafId {
    pub txid: Txid,
    pub index: u32,
}

impl LeafId {
    pub fn append_to_vec(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(&self.txid.0);
        v.extend_from_slice(&self.index.to_le_bytes());
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();

        self.append_to_vec(&mut v);

        v
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() < 36 {
            return Err(Error::WrongLengthForLeafId(slice.len()));
        }

        let txid = Txid::from_slice(&slice[..32])?;
        let index = u32::from_le_bytes(slice[32..36].try_into().unwrap());

        Ok(Self { txid, index })
    }
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
