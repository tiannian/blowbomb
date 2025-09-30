use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use bbm_primitives::{FilledTransaction, Leaf, LeafId, Transaction};

use crate::LeafStorage;

#[derive(Default)]
pub struct TransactionChecker {
    buffer_leaf_ids: BTreeSet<LeafId>,
    used_buffer_leaf_ids: BTreeSet<LeafId>,
    operators: BTreeMap<LeafId, Leaf>,
}

impl TransactionChecker {
    pub async fn check_leaf_id<S>(
        &mut self,
        leaf_storage: &S,
        transaction: Transaction,
    ) -> Result<FilledTransaction>
    where
        S: LeafStorage,
    {
        let txid = transaction.hash()?;

        if transaction.inputs.len() != transaction.unlockers.len() {
            log::warn!("Input length mismatch: {:?}", transaction);
            return Err(anyhow::anyhow!(
                "Input length mismatch for txid: {:?}",
                txid
            ));
        }

        for (i, _leaf) in transaction.outputs.iter().enumerate() {
            self.buffer_leaf_ids.insert(LeafId {
                txid: txid.clone(),
                index: i as u32,
            });
        }

        let mut filled_tx_inputs = Vec::new();

        for leaf_id in &transaction.inputs {
            if self.used_buffer_leaf_ids.contains(leaf_id) {
                return Err(anyhow::anyhow!("Input leaf id already used: {:?}", leaf_id));
            }

            // if inputs in buffer_leaf_ids or in storage, pass validate
            if self.buffer_leaf_ids.contains(&leaf_id) {
                self.used_buffer_leaf_ids.insert(leaf_id.clone());
                continue;
            }

            let leaf = leaf_storage
                .get_leaf(&leaf_id)
                .await?
                .ok_or(anyhow::anyhow!(
                    "Input leaf not found in storage: {:?}",
                    leaf_id
                ))?;

            self.used_buffer_leaf_ids.insert(leaf_id.clone());

            if let Some(operator) = leaf.operator.clone() {
                let operator_leaf = leaf_storage.get_leaf(&operator).await?;
                if let Some(operator_leaf) = operator_leaf {
                    self.operators.insert(operator, operator_leaf);
                } else {
                    return Err(anyhow::anyhow!(
                        "Operator leaf not found in storage: {:?}",
                        operator
                    ));
                }
            }

            filled_tx_inputs.push(leaf);
        }

        Ok(FilledTransaction {
            inputs: filled_tx_inputs,
            unlockers: transaction.unlockers,
            outputs: transaction.outputs,
        })
    }
}
