use crate::{Bytes, Error, Leaf, LeafId, Result, Txid};

pub struct Transaction {
    pub version: u8,
    pub nonce: u64,
    pub inputs: Vec<LeafId>,
    pub unlocker: Vec<Bytes>,
    pub outputs: Vec<Leaf>,
}

impl Transaction {
    pub fn append_to_vec(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(&self.version.to_be_bytes());
        v.extend_from_slice(&self.nonce.to_be_bytes());
        v.extend_from_slice(&self.inputs.len().to_be_bytes());
        v.extend_from_slice(&self.outputs.len().to_be_bytes());

        for input in &self.inputs {
            input.append_to_vec(v);
        }

        for unlocker in &self.unlocker {
            v.extend_from_slice(&unlocker.0);
        }

        for output in &self.outputs {
            v.extend_from_slice(&output.to_vec());
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();
        self.append_to_vec(&mut v);
        v
    }

    // pub fn from_slice(slice: &[u8]) -> Result<Self> {
    //     if slice.len() < 21 {
    //         return Err(Error::WrongLengthForTx(slice.len(), 21));
    //     }

    //     let version = slice[0];
    //     let nonce = u64::from_be_bytes(slice[1..9].try_into().unwrap());

    //     let inputs_len = u32::from_be_bytes(slice[9..13].try_into().unwrap()) as usize;
    //     let outputs_len = u32::from_be_bytes(slice[13..17].try_into().unwrap()) as usize;
    //     let unlocker_len = inputs_len;

    //     let mut inputs = Vec::new();

    //     for i in 0..inputs_len {
    //         let input = LeafId::from_slice(&slice[17 + i * 36..])?;
    //         inputs.push(input);
    //     }

    //     let mut unlocker = Vec::new();

    //     for i in 0..unlocker_len {
    //         let unlocker = Bytes::from_slice(&slice[17 + inputs_len + i * 4..])?;
    //         unlocker.push(unlocker);
    //     }

    //     let mut outputs = Vec::new();

    //     Ok(Self {
    //         version,
    //         nonce,
    //         inputs,
    //         unlocker,
    //         outputs,
    //     })
    // }
}
