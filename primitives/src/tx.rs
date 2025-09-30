use crate::{Address, Bytes, Error, IndexKey, Leaf, LeafId, Result, Txid};

pub struct Transaction {
    pub version: u8,
    pub nonce: u64,
    pub inputs: Vec<LeafId>,
    pub unlockers: Vec<Bytes>,
    pub outputs: Vec<Leaf>,
}

impl Transaction {
    pub fn append_to_vec(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(&self.version.to_be_bytes());
        v.extend_from_slice(&self.nonce.to_be_bytes());

        let input_count = self.inputs.len() as u32;
        let output_count = self.outputs.len() as u32;
        v.extend_from_slice(&input_count.to_be_bytes());
        v.extend_from_slice(&output_count.to_be_bytes());

        for unlocker in &self.unlockers {
            let unlocker_len = unlocker.0.len() as u32;
            v.extend_from_slice(&unlocker_len.to_be_bytes());
        }

        for output in &self.outputs {
            let output_len = output.data.0.len() as u32;
            v.extend_from_slice(&output_len.to_be_bytes());
        }

        for input in &self.inputs {
            v.extend_from_slice(&input.txid.0);
            v.extend_from_slice(&input.index.to_be_bytes());
        }

        for unlocker in &self.unlockers {
            v.extend_from_slice(&unlocker.0);
        }

        for output in &self.outputs {
            v.extend_from_slice(&output.version.to_be_bytes());
            v.extend_from_slice(&output.owner.0);
            v.extend_from_slice(&output.index_key.0);
            v.extend_from_slice(&output.operator.txid.0);
            v.extend_from_slice(&output.operator.index.to_be_bytes());
            v.extend_from_slice(&output.data.0);
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();
        self.append_to_vec(&mut v);
        v
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() < 17 {
            return Err(Error::WrongLengthForTx(slice.len(), 21));
        }

        let version = slice[0];
        let nonce = u64::from_be_bytes(slice[1..9].try_into().unwrap());

        let inputs_len = u32::from_be_bytes(slice[9..13].try_into().unwrap()) as usize;
        let outputs_len = u32::from_be_bytes(slice[13..17].try_into().unwrap()) as usize;

        let inputs_length_pos = 1 + 9 + 4 + 4;
        let mut inputs_begin_pos = inputs_length_pos + inputs_len * 4;

        let mut inputs = Vec::new();
        let mut unlockers = Vec::new();

        for _ in 0..inputs_len {
            // Get unlocker length for input
            let begin = inputs_length_pos;
            let end = begin + 4;
            let unlocker_len = u32::from_be_bytes(slice[begin..end].try_into().unwrap()) as usize;

            // Get txid for input
            let begin = inputs_begin_pos;
            let end = begin + 32;
            let txid = Txid::from_slice(&slice[begin..end])?;

            // Get index for input
            let begin = end;
            let end = begin + 4;
            let index = u32::from_be_bytes(slice[begin..end].try_into().unwrap());

            inputs.push(LeafId { txid, index });

            // Get unlocker for input
            let begin = end;
            let end = begin + unlocker_len;
            let unlocker = Bytes::from_slice(&slice[begin..end]);

            unlockers.push(unlocker);

            inputs_begin_pos = end;
        }

        let mut outputs = Vec::new();

        let outputs_length_pos = inputs_begin_pos;
        let mut outputs_begin_pos = outputs_length_pos + outputs_len * 4;

        for _ in 0..outputs_len {
            // Get output length for output
            let begin = outputs_length_pos;
            let end = begin + 4;
            let output_len = u32::from_be_bytes(slice[begin..end].try_into().unwrap()) as usize;

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
            unlockers,
            outputs,
        })
    }
}
