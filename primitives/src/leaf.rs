use crate::{Address, IndexKey, LeafId};

pub struct Leaf {
    pub leaf_id: LeafId,
    pub owner: Address,
    pub index_key: IndexKey,
    pub operator: LeafId,
    pub data: Vec<u8>,
}
