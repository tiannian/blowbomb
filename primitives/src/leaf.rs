use crate::{Address, Error, IndexKey, LeafId, Result};

pub struct Leaf {
    pub version: u8,
    pub owner: Address,
    pub index_key: IndexKey,
    pub operator: LeafId,
    pub data: Vec<u8>,
}

impl Leaf {
    pub fn append_to_vec(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(&self.version.to_be_bytes());
        v.extend_from_slice(&self.owner.0);
        v.extend_from_slice(&self.index_key.0);
        self.operator.append_to_vec(v);
        v.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        v.extend_from_slice(&self.data);
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();
        self.append_to_vec(&mut v);
        v
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let version = slice[0];
        let owner = Address::from_slice(&slice[33..53])?;
        let index_key = IndexKey::from_slice(&slice[53..85])?;
        let operator = LeafId::from_slice(&slice[85..121])?;
        let data_len = u32::from_be_bytes(slice[121..125].try_into().unwrap()) as usize;

        if slice.len() < 125 + data_len {
            return Err(Error::WrongLengthForLeaf(slice.len(), 125 + data_len));
        }

        let data = slice[125..(125 + data_len)].to_vec();

        Ok(Self {
            version,
            owner,
            index_key,
            operator,
            data,
        })
    }
}
