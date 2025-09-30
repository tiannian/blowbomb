use crate::{Address, Bytes, IndexKey, LeafId};

#[derive(Debug, PartialEq)]
pub struct Leaf {
    pub version: u8,
    pub owner: Address,
    pub index_key: IndexKey,
    pub operator: LeafId,
    pub data: Bytes,
}
