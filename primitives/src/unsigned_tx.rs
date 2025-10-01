use sha3::Digest;

use crate::{Address, Bytes, Error, IndexKey, Leaf, LeafId, Result, Txid};

#[derive(Debug, PartialEq)]
pub struct UnsignedTransaction {
    pub version: u8,
    pub nonce: u64,
    pub inputs: Vec<LeafId>,
    pub outputs: Vec<Leaf>,
}

impl UnsignedTransaction {
    pub fn append_to_vec(&self, v: &mut Vec<u8>) -> Result<()> {
        v.extend_from_slice(&self.version.to_be_bytes());
        v.extend_from_slice(&self.nonce.to_be_bytes());

        let input_count = self.inputs.len() as u32;
        let output_count = self.outputs.len() as u32;
        v.extend_from_slice(&input_count.to_be_bytes());
        v.extend_from_slice(&output_count.to_be_bytes());

        for output in &self.outputs {
            let output_len = output.data.0.len() as u32;
            v.extend_from_slice(&output_len.to_be_bytes());
        }

        for input in &self.inputs {
            v.extend_from_slice(&input.txid.0);
            v.extend_from_slice(&input.index.to_be_bytes());
        }

        for output in &self.outputs {
            v.extend_from_slice(&output.version.to_be_bytes());
            v.extend_from_slice(&output.owner.0);
            v.extend_from_slice(&output.index_key.0);

            if let Some(operator) = &output.operator {
                v.extend_from_slice(&operator.txid.0);
                v.extend_from_slice(&operator.index.to_be_bytes());
            } else {
                v.extend_from_slice(&[0u8; 32]);
                v.extend_from_slice(&[0u8; 4]);
            }
            v.extend_from_slice(&output.data.0);
        }

        Ok(())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut v = Vec::new();
        self.append_to_vec(&mut v)?;
        Ok(v)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        const HEADER_LENGTH: usize = 1 + 9 + 4 + 4 - 1;

        if slice.len() < HEADER_LENGTH {
            return Err(Error::WrongLengthForTx(slice.len(), HEADER_LENGTH));
        }

        let version = slice[0];
        let nonce = u64::from_be_bytes(slice[1..9].try_into().unwrap());

        let inputs_count = u32::from_be_bytes(slice[9..13].try_into().unwrap()) as usize;
        let outputs_count = u32::from_be_bytes(slice[13..17].try_into().unwrap()) as usize;

        let inputs_length_pos = HEADER_LENGTH;
        let mut inputs_begin_pos = inputs_length_pos + outputs_count * 4;

        let mut inputs = Vec::new();

        for _ in 0..inputs_count {
            // Get txid for input
            let begin = inputs_begin_pos;
            let end = begin + 32;
            let txid = Txid::from_slice(&slice[begin..end])?;

            // Get index for input
            let begin = end;
            let end = begin + 4;
            let index = u32::from_be_bytes(slice[begin..end].try_into().unwrap());

            inputs.push(LeafId { txid, index });

            inputs_begin_pos = end;
        }

        let mut outputs = Vec::new();

        let outputs_length_pos = inputs_length_pos;
        let mut outputs_begin_pos = inputs_begin_pos;

        for i in 0..outputs_count {
            // Get output length for output
            let begin = outputs_length_pos + i * 4;
            let end = begin + 4;
            let bytes = slice[begin..end].try_into().unwrap();
            let output_len = u32::from_be_bytes(bytes) as usize;

            // Get output version
            let begin = outputs_begin_pos;
            let end = begin + 1;
            let version = slice[begin];

            // Get output owner
            let begin = end;
            let end = begin + 20;
            let owner = Address::from_slice(&slice[begin..end])?;

            // Get output index key
            let begin = end;
            let end = begin + 32;
            let index_key = IndexKey::from_slice(&slice[begin..end])?;

            // Get output operator txid
            let begin = end;
            let end = begin + 32;
            let operator_txid = Txid::from_slice(&slice[begin..end])?;

            // Get output operator index
            let begin = end;
            let end = begin + 4;
            let operator_index = u32::from_be_bytes(slice[begin..end].try_into().unwrap());

            let operator = LeafId {
                txid: operator_txid,
                index: operator_index,
            };

            let operator = if operator.is_some() {
                Some(operator)
            } else {
                None
            };

            // Get output data
            let begin = end;
            let end = begin + output_len;
            let data = Bytes::from_slice(&slice[begin..end]);

            outputs.push(Leaf {
                version,
                owner,
                index_key,
                operator,
                data,
            });

            outputs_begin_pos = end;
        }

        Ok(Self {
            version,
            nonce,
            inputs,
            outputs,
        })
    }

    pub fn hash(&self) -> Result<Txid> {
        let mut hasher = sha3::Sha3_256::new();

        let bytes = self.to_vec()?;
        hasher.update(&bytes);

        let hash = hasher.finalize();

        Ok(Txid::from_slice(&hash)?)
    }
}

pub struct FilledTransaction {
    pub inputs: Vec<Leaf>,
    pub unlockers: Vec<Bytes>,
    pub outputs: Vec<Leaf>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FixedBytes;

    #[test]
    fn test_transaction_serialization_deserialization() {
        // Create test transaction
        let tx1 = UnsignedTransaction {
            version: 1,
            nonce: 12345,
            inputs: vec![
                LeafId {
                    txid: FixedBytes([1u8; 32]),
                    index: 0,
                },
                LeafId {
                    txid: FixedBytes([2u8; 32]),
                    index: 1,
                },
            ],
            outputs: vec![Leaf {
                version: 1,
                owner: FixedBytes([3u8; 20]),
                index_key: FixedBytes([4u8; 32]),
                operator: Some(LeafId {
                    txid: FixedBytes([5u8; 32]),
                    index: 2,
                }),
                data: Bytes(vec![60, 70, 80, 90]),
            }],
        };

        // Serialize with to_vec
        let serialized = tx1.to_vec().expect("Failed to serialize");

        // Deserialize with from_slice
        let tx2 = UnsignedTransaction::from_slice(&serialized).expect("Failed to deserialize");

        // Compare if they are equal
        assert_eq!(
            tx1, tx2,
            "Transaction should be equal after serialization and deserialization"
        );
    }

    #[test]
    fn test_transaction_empty_inputs_outputs() {
        // Test empty inputs and outputs
        let tx1 = UnsignedTransaction {
            version: 2,
            nonce: 999,
            inputs: vec![],
            outputs: vec![],
        };

        let serialized = tx1.to_vec().expect("Failed to serialize");
        let tx2 = UnsignedTransaction::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(
            tx1, tx2,
            "Empty transaction should be equal after serialization and deserialization"
        );
    }

    #[test]
    fn test_transaction_multiple_outputs() {
        // Test multiple outputs
        let tx1 = UnsignedTransaction {
            version: 3,
            nonce: 54321,
            inputs: vec![LeafId {
                txid: FixedBytes([10u8; 32]),
                index: 5,
            }],
            outputs: vec![
                Leaf {
                    version: 1,
                    owner: FixedBytes([11u8; 20]),
                    index_key: FixedBytes([12u8; 32]),
                    operator: Some(LeafId {
                        txid: FixedBytes([13u8; 32]),
                        index: 10,
                    }),
                    data: Bytes(vec![100, 101, 102]),
                },
                Leaf {
                    version: 2,
                    owner: FixedBytes([21u8; 20]),
                    index_key: FixedBytes([22u8; 32]),
                    operator: Some(LeafId {
                        txid: FixedBytes([23u8; 32]),
                        index: 20,
                    }),
                    data: Bytes(vec![200]),
                },
            ],
        };

        let serialized = tx1.to_vec().expect("Failed to serialize");
        let tx2 = UnsignedTransaction::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(
            tx1, tx2,
            "Transaction with multiple outputs should be equal after serialization and deserialization"
        );
    }
}
