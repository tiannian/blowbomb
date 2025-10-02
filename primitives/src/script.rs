use sha3::{Digest, Sha3_256};

use crate::{Address, Error, Result};

pub enum UnlockScriptType {
    Empty,
    Wasm,
}

impl UnlockScriptType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Empty),
            1 => Some(Self::Wasm),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Empty => 0,
            Self::Wasm => 1,
        }
    }
}

pub struct UnlockScript {
    pub version: u8,
    pub ty: UnlockScriptType,
    pub code_leaf: [u8; 32],
    pub args: Vec<u8>,
}

impl UnlockScript {
    pub fn append_to_vec(&self, v: &mut Vec<u8>) -> Result<()> {
        v.extend_from_slice(&self.version.to_be_bytes());
        v.extend_from_slice(&(self.args.len() as u32).to_be_bytes());
        v.extend_from_slice(&self.ty.to_u8().to_be_bytes());
        v.extend_from_slice(&self.code_leaf);
        v.extend_from_slice(&self.args);

        Ok(())
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() < 38 {
            return Err(Error::WrongLengthForUnlockScript(slice.len(), 38));
        }

        let args_len = u32::from_be_bytes(slice[1..5].try_into().unwrap());

        if slice.len() < 38 + args_len as usize {
            return Err(Error::WrongLengthForUnlockScript(
                slice.len(),
                38 + args_len as usize,
            ));
        }

        let version = u8::from_be_bytes(slice[0..1].try_into().unwrap());
        let ty = UnlockScriptType::from_u8(slice[5]).unwrap();
        let code_leaf = slice[6..38].try_into().unwrap();
        let args = slice[38..38 + args_len as usize].to_vec();

        Ok(Self {
            version,
            ty,
            code_leaf,
            args,
        })
    }

    pub fn address(&self) -> Result<Address> {
        let mut hasher = Sha3_256::new();
        hasher.update(&[self.version, self.ty.to_u8()]);
        hasher.update(&self.code_leaf);
        hasher.update(&self.args);

        let hash = hasher.finalize();
        Address::from_slice(&hash[..20])
    }
}
