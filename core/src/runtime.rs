use anyhow::Result;
use bbm_primitives::Transaction;

use crate::{Storage, TransactionChecker};

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

    pub async fn batch_execute_transaction(&self, transactions: Vec<Transaction>) -> Result<()> {
        let leaf_storage = self.storage.open_leaf_storage()?;

        let mut checker = TransactionChecker::default();

        // check leaf_id
        let mut filled_txs = Vec::new();
        for transaction in transactions {
            let filled_transaction = checker.check_leaf_id(&leaf_storage, transaction).await?;
            filled_txs.push(filled_transaction);
        }

        // check scripts

        // check operators

        // append all leafs and mark spent

        Ok(())
    }
}
