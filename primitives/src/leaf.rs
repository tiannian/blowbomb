use crate::{Address, Bytes, Error, IndexKey, LeafId, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct Leaf {
    pub version: u8,
    pub nonce: u64,
    pub owner: Address,
    pub index: IndexKey,
    pub operator: Option<LeafId>,
    pub data: Bytes,
}

impl Leaf {
    pub fn append_to_vec(&self, v: &mut Vec<u8>) -> Result<()> {
        v.extend_from_slice(&self.version.to_be_bytes());
        v.extend_from_slice(&(self.data.0.len() as u32).to_be_bytes());
        v.extend_from_slice(&self.nonce.to_be_bytes());
        v.extend_from_slice(&self.owner.0);
        v.extend_from_slice(&self.index.0);

        let operator = if let Some(operator) = &self.operator {
            operator.0
        } else {
            [0u8; 32]
        };

        v.extend_from_slice(&operator);
        v.extend_from_slice(&self.data.0);
        Ok(())
    }
}

pub struct LeafParser<T> {
    inner: T,
}

impl<T> LeafParser<T>
where
    T: AsRef<[u8]>,
{
    pub fn new(inner: T) -> Result<Self> {
        check_leaf_length(inner.as_ref())?;

        Ok(Self { inner })
    }
}

const LEAF_HEADER_LENGTH: usize = 1 + 8 + 20 + 32 + 32 + 4;

fn check_leaf_length(slice: &[u8]) -> Result<()> {
    let len = slice.len();

    if len < LEAF_HEADER_LENGTH {
        return Err(Error::WrongLengthForLeaf(len, 97));
    }

    let data_len = u32::from_be_bytes(slice[1..5].try_into().unwrap()) as usize;

    if len < LEAF_HEADER_LENGTH + data_len {
        return Err(Error::WrongLengthForLeaf(
            len,
            LEAF_HEADER_LENGTH + data_len,
        ));
    }

    Ok(())
}

impl<'a> LeafParser<&'a [u8]> {
    pub fn version(&self) -> u8 {
        self.inner[0]
    }

    pub fn data_len(&self) -> u32 {
        u32::from_be_bytes(self.inner[1..5].try_into().unwrap())
    }

    pub fn leaf_len(&self) -> usize {
        LEAF_HEADER_LENGTH + self.data_len() as usize
    }

    pub fn nonce(&self) -> u64 {
        u64::from_be_bytes(self.inner[5..13].try_into().unwrap())
    }

    pub fn owner(&self) -> &[u8; 20] {
        let owner = &self.inner[13..33];
        owner.try_into().unwrap()
    }

    pub fn index(&self) -> &[u8; 32] {
        let index = &self.inner[33..65];
        index.try_into().unwrap()
    }

    pub fn operator(&self) -> &[u8; 32] {
        let operator = &self.inner[65..97];
        operator.try_into().unwrap()
    }

    pub fn data(&self) -> &[u8] {
        let data_len = self.data_len() as usize;
        &self.inner[97..97 + data_len]
    }

    pub fn to_leaf(&self) -> Result<Leaf> {
        Ok(Leaf {
            version: self.version(),
            nonce: self.nonce(),
            owner: Address::from_slice(self.owner())?,
            index: IndexKey::from_slice(self.index())?,
            operator: if self.operator() == &[0u8; 32] {
                None
            } else {
                Some(LeafId::from_slice(self.operator())?)
            },
            data: Bytes::from_slice(self.data()),
        })
    }
}

impl<'a> LeafParser<&'a mut [u8]> {
    pub fn version(&self) -> u8 {
        self.inner[0]
    }

    pub fn data_len(&self) -> u32 {
        u32::from_be_bytes(self.inner[1..5].try_into().unwrap())
    }

    pub fn leaf_len(&self) -> usize {
        LEAF_HEADER_LENGTH + self.data_len() as usize
    }

    pub fn nonce(&self) -> u64 {
        u64::from_be_bytes(self.inner[5..13].try_into().unwrap())
    }

    pub fn owner(&self) -> &[u8; 20] {
        let owner = &self.inner[13..33];
        owner.try_into().unwrap()
    }

    pub fn index(&self) -> &[u8; 32] {
        let index = &self.inner[33..65];
        index.try_into().unwrap()
    }

    pub fn operator(&self) -> &[u8; 32] {
        let operator = &self.inner[65..97];
        operator.try_into().unwrap()
    }

    pub fn data(&self) -> &[u8] {
        let data_len = self.data_len() as usize;
        &self.inner[97..97 + data_len]
    }

    pub fn set_version(&mut self, version: u8) {
        self.inner[0] = version;
    }

    pub fn set_data_len(&mut self, data_len: u32) {
        self.inner[1..5].copy_from_slice(&data_len.to_be_bytes());
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.inner[5..13].copy_from_slice(&nonce.to_be_bytes());
    }

    pub fn set_owner(&mut self, owner: &[u8; 20]) {
        self.inner[13..33].copy_from_slice(owner);
    }

    pub fn set_index(&mut self, index: &[u8; 32]) {
        self.inner[33..65].copy_from_slice(index);
    }

    pub fn set_operator(&mut self, operator: &[u8; 32]) {
        self.inner[65..97].copy_from_slice(operator);
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        let data_len = self.data_len() as usize;
        &mut self.inner[97..97 + data_len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_parser() {
        let leaf = Leaf {
            version: 1,
            nonce: 12345,
            owner: Address::from_slice(&[1u8; 20]).unwrap(),
            index: IndexKey::from_slice(&[2u8; 32]).unwrap(),
            operator: None,
            data: Bytes::from_slice(&[3u8; 32]),
        };

        let mut bytes = Vec::new();
        leaf.append_to_vec(&mut bytes).unwrap();
        let bytes_ref = bytes.as_slice();
        let parsed = LeafParser::new(bytes_ref).unwrap();
        assert_eq!(leaf, parsed.to_leaf().unwrap());
    }
}
