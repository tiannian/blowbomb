use anyhow::Result;
use async_trait::async_trait;
use bbm_primitives::{IndexKey, Leaf, LeafId, LeafWithId};

pub trait CommittableStorage {
    fn commit(self, version: u64) -> Result<()>;
}

#[async_trait]
pub trait LeafStorage: CommittableStorage {
    async fn store_leaf(&self, leaf_id: &LeafId, leaf: Leaf) -> Result<()>;

    async fn get_leaf(&self, leaf_id: &LeafId) -> Result<Option<Leaf>>;

    async fn get_leaf_by_index_key(&self, index_key: &IndexKey) -> Result<Vec<LeafWithId>>;

    async fn mark_leaf_as_spent(&self, leaf_id: &LeafId) -> Result<()>;

    async fn purge_spent_leaves(&self) -> Result<()>;
}

#[async_trait]
pub trait Storage {
    type LeafStorage: LeafStorage;

    async fn revert_to_version(&self, version: u64) -> Result<()>;

    fn open_leaf_storage(&self) -> Result<Self::LeafStorage>;
}
