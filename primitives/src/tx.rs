use crate::{Bytes, Result, UnsignedTransaction};

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub unsigned: UnsignedTransaction,
    pub unlockers: Vec<Bytes>,
}

impl Transaction {
    pub fn append_to_vec(&self, v: &mut Vec<u8>) -> Result<()> {
        self.unsigned.append_to_vec(v)?;

        let unlocker_count = self.unlockers.len() as u32;

        for unlocker in self.unlockers.iter().rev() {
            v.extend_from_slice(&unlocker.0);
        }

        for unlocker in self.unlockers.iter().rev() {
            let unlocker_len = unlocker.0.len() as u32;
            println!("unlocker_len: {}", unlocker_len);
            v.extend_from_slice(&unlocker_len.to_be_bytes());
        }

        v.extend_from_slice(&unlocker_count.to_be_bytes());

        Ok(())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut v = Vec::new();
        self.append_to_vec(&mut v)?;
        Ok(v)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let unsigned = UnsignedTransaction::from_slice(slice)?;
        let unlocker_count =
            u32::from_be_bytes(slice[slice.len() - 4..slice.len()].try_into().unwrap()) as usize;

        let mut unlockers = vec![Bytes::default(); unlocker_count];

        let mut unlockers_length_pos = slice.len() - 4;
        let mut unlockers_begin_pos = slice.len() - 4 - unlocker_count * 4;

        for i in 0..unlocker_count {
            let begin = unlockers_length_pos;
            let end = begin + 4;
            let unlocker_len = u32::from_be_bytes(slice[begin..end].try_into().unwrap()) as usize;
            println!("unlocker_len: {}", unlocker_len);
            unlockers_length_pos = begin - 4;

            let begin = unlockers_begin_pos;
            let end = begin - unlocker_len;

            println!("begin: {}, end: {}", begin, end);

            let data = &slice[end..begin];
            let unlocker = Bytes::from_slice(data);

            println!("unlocker: {:?}", unlocker);

            unlockers_begin_pos = end;
        }

        // unlockers.res

        Ok(Self {
            unsigned,
            unlockers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FixedBytes, Leaf, LeafId};

    #[test]
    fn test_transaction_serialization_deserialization() {
        // Create test transaction with multiple unlockers
        let tx1 = Transaction {
            unsigned: UnsignedTransaction {
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
            },
            unlockers: vec![Bytes(vec![10, 20, 30]), Bytes(vec![40, 50])],
        };

        // Serialize with to_vec
        let serialized = tx1.to_vec().expect("Failed to serialize");

        // Deserialize with from_slice
        let tx2 = Transaction::from_slice(&serialized).expect("Failed to deserialize");

        // Compare if they are equal
        assert_eq!(
            tx1, tx2,
            "Transaction should be equal after serialization and deserialization"
        );
    }

    #[test]
    fn test_transaction_empty_unlockers() {
        // Test transaction with empty unlockers
        let tx1 = Transaction {
            unsigned: UnsignedTransaction {
                version: 2,
                nonce: 999,
                inputs: vec![],
                outputs: vec![],
            },
            unlockers: vec![],
        };

        let serialized = tx1.to_vec().expect("Failed to serialize");
        let tx2 = Transaction::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(
            tx1, tx2,
            "Transaction with empty unlockers should be equal after serialization and deserialization"
        );
    }

    #[test]
    fn test_transaction_single_unlocker() {
        // Test transaction with single unlocker
        let tx1 = Transaction {
            unsigned: UnsignedTransaction {
                version: 1,
                nonce: 5000,
                inputs: vec![LeafId {
                    txid: FixedBytes([7u8; 32]),
                    index: 3,
                }],
                outputs: vec![Leaf {
                    version: 1,
                    owner: FixedBytes([8u8; 20]),
                    index_key: FixedBytes([9u8; 32]),
                    operator: None,
                    data: Bytes(vec![100, 101, 102, 103]),
                }],
            },
            unlockers: vec![Bytes(vec![200, 201, 202, 203, 204])],
        };

        let serialized = tx1.to_vec().expect("Failed to serialize");
        let tx2 = Transaction::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(
            tx1, tx2,
            "Transaction with single unlocker should be equal after serialization and deserialization"
        );
    }

    #[test]
    fn test_transaction_multiple_unlockers() {
        // Test transaction with multiple unlockers of varying sizes
        let tx1 = Transaction {
            unsigned: UnsignedTransaction {
                version: 3,
                nonce: 99999,
                inputs: vec![
                    LeafId {
                        txid: FixedBytes([11u8; 32]),
                        index: 5,
                    },
                    LeafId {
                        txid: FixedBytes([12u8; 32]),
                        index: 6,
                    },
                    LeafId {
                        txid: FixedBytes([13u8; 32]),
                        index: 7,
                    },
                ],
                outputs: vec![
                    Leaf {
                        version: 1,
                        owner: FixedBytes([14u8; 20]),
                        index_key: FixedBytes([15u8; 32]),
                        operator: Some(LeafId {
                            txid: FixedBytes([16u8; 32]),
                            index: 8,
                        }),
                        data: Bytes(vec![1, 2, 3]),
                    },
                    Leaf {
                        version: 2,
                        owner: FixedBytes([17u8; 20]),
                        index_key: FixedBytes([18u8; 32]),
                        operator: None,
                        data: Bytes(vec![4, 5]),
                    },
                ],
            },
            unlockers: vec![
                Bytes(vec![255]),
                Bytes(vec![254, 253, 252]),
                Bytes(vec![251, 250, 249, 248, 247, 246]),
            ],
        };

        let serialized = tx1.to_vec().expect("Failed to serialize");
        let tx2 = Transaction::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(
            tx1, tx2,
            "Transaction with multiple unlockers should be equal after serialization and deserialization"
        );
    }

    #[test]
    fn test_transaction_large_unlocker() {
        // Test transaction with a large unlocker
        let large_data = vec![42u8; 1000];
        let tx1 = Transaction {
            unsigned: UnsignedTransaction {
                version: 1,
                nonce: 777,
                inputs: vec![LeafId {
                    txid: FixedBytes([20u8; 32]),
                    index: 10,
                }],
                outputs: vec![Leaf {
                    version: 1,
                    owner: FixedBytes([21u8; 20]),
                    index_key: FixedBytes([22u8; 32]),
                    operator: Some(LeafId {
                        txid: FixedBytes([23u8; 32]),
                        index: 11,
                    }),
                    data: Bytes(vec![1, 2, 3, 4, 5]),
                }],
            },
            unlockers: vec![Bytes(large_data.clone())],
        };

        let serialized = tx1.to_vec().expect("Failed to serialize");
        let tx2 = Transaction::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(
            tx1, tx2,
            "Transaction with large unlocker should be equal after serialization and deserialization"
        );
        assert_eq!(
            tx2.unlockers[0].0.len(),
            1000,
            "Unlocker size should be preserved"
        );
    }

    #[test]
    fn test_transaction_empty_unlocker_bytes() {
        // Test transaction with unlockers containing empty byte arrays
        let tx1 = Transaction {
            unsigned: UnsignedTransaction {
                version: 1,
                nonce: 111,
                inputs: vec![LeafId {
                    txid: FixedBytes([25u8; 32]),
                    index: 0,
                }],
                outputs: vec![Leaf {
                    version: 1,
                    owner: FixedBytes([26u8; 20]),
                    index_key: FixedBytes([27u8; 32]),
                    operator: None,
                    data: Bytes(vec![99]),
                }],
            },
            unlockers: vec![Bytes(vec![]), Bytes(vec![1]), Bytes(vec![])],
        };

        let serialized = tx1.to_vec().expect("Failed to serialize");
        let tx2 = Transaction::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(
            tx1, tx2,
            "Transaction with empty byte unlockers should be equal after serialization and deserialization"
        );
    }

    #[test]
    fn test_transaction_serialization_format() {
        // Verify the serialization format structure
        let tx = Transaction {
            unsigned: UnsignedTransaction {
                version: 1,
                nonce: 100,
                inputs: vec![],
                outputs: vec![],
            },
            unlockers: vec![Bytes(vec![1, 2, 3]), Bytes(vec![4, 5])],
        };

        let serialized = tx.to_vec().expect("Failed to serialize");

        // Check unlocker count at the end (last 4 bytes)
        let unlocker_count_bytes = &serialized[serialized.len() - 4..];
        let unlocker_count = u32::from_be_bytes(unlocker_count_bytes.try_into().unwrap());
        assert_eq!(
            unlocker_count, 2,
            "Unlocker count should be stored at the end"
        );

        // Verify deserialization works
        let tx2 = Transaction::from_slice(&serialized).expect("Failed to deserialize");
        assert_eq!(
            tx.unlockers.len(),
            tx2.unlockers.len(),
            "Unlocker count should match"
        );
    }

    #[test]
    fn test_transaction_roundtrip_consistency() {
        // Test multiple rounds of serialization/deserialization
        let tx1 = Transaction {
            unsigned: UnsignedTransaction {
                version: 5,
                nonce: 54321,
                inputs: vec![LeafId {
                    txid: FixedBytes([30u8; 32]),
                    index: 15,
                }],
                outputs: vec![Leaf {
                    version: 3,
                    owner: FixedBytes([31u8; 20]),
                    index_key: FixedBytes([32u8; 32]),
                    operator: Some(LeafId {
                        txid: FixedBytes([33u8; 32]),
                        index: 16,
                    }),
                    data: Bytes(vec![77, 88, 99]),
                }],
            },
            unlockers: vec![Bytes(vec![111, 222, 77])],
        };

        // First roundtrip
        let serialized1 = tx1.to_vec().expect("Failed to serialize first time");
        let tx2 = Transaction::from_slice(&serialized1).expect("Failed to deserialize first time");

        // Second roundtrip
        let serialized2 = tx2.to_vec().expect("Failed to serialize second time");
        let tx3 = Transaction::from_slice(&serialized2).expect("Failed to deserialize second time");

        // All should be equal
        assert_eq!(tx1, tx2, "First roundtrip should preserve equality");
        assert_eq!(tx2, tx3, "Second roundtrip should preserve equality");
        assert_eq!(
            serialized1, serialized2,
            "Serialized bytes should be identical"
        );
    }
}
