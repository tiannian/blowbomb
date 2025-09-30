use anyhow::Result;
use bbm_primitives::Transaction;

use crate::Storage;

pub struct Runtime<S> {
    storage: S,
}

impl<S> Runtime<S>
where
    S: Storage,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub async fn append_transaction_batch(&self, transactions: Vec<Transaction>) -> Result<()> {
        let leaf_storage = self.storage.open_leaf_storage()?;

        Ok(())
    }

    async fn append_transaction(&self, transaction: Transaction) -> Result<()> {
        Ok(())
    }
}
