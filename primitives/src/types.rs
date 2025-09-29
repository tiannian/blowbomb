use crate::{Error, Result};

pub struct Txid(pub [u8; 32]);

pub struct LeafId {
    pub txid: Txid,
    pub index: u32,
}

impl LeafId {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&self.txid.0);
        v.extend_from_slice(&self.index.to_le_bytes());
        v
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() < 36 {
            return Err(Error::WrongLengthForLeafId(slice.len()));
        }

        let txid = Txid(slice[..32].try_into().unwrap());
        let index = u32::from_le_bytes(slice[32..36].try_into().unwrap());

        Ok(Self { txid, index })
    }
}

pub struct IndexKey(pub [u8; 32]);

pub struct Address(pub [u8; 20]);
