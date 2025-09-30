use crate::{Address, Bytes, IndexKey, LeafId};

#[derive(Clone, Debug, PartialEq)]
pub struct Leaf {
    pub version: u8,
    pub owner: Address,
    pub index_key: IndexKey,
    pub operator: Option<LeafId>,
    pub data: Bytes,
}

#[derive(Debug, PartialEq)]
pub struct LeafWithId {
    pub leaf: Leaf,
    pub id: LeafId,
}
