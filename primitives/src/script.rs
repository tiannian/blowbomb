use sha3::Digest;

use crate::{Address, Error, Result};

pub enum ScriptType {
    Empty,
    Wasm,
}

impl ScriptType {
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

pub struct Script<'a> {
    version: u8,
    script_type: ScriptType,
    code: &'a [u8],
    args: &'a [u8],
    data: &'a [u8],
}

impl<'a> Script<'a> {
    pub fn from_slice(slice: &'a [u8]) -> Result<Self> {
        if slice.len() < 10 {
            return Err(Error::WrongLengthForScript(slice.len(), 14));
        }

        let version = slice[0];
        let script_type =
            ScriptType::from_u8(slice[1]).ok_or(Error::WrongLengthForScript(slice.len(), 2))?;

        let code_len_bytes = [slice[2], slice[3], slice[4], slice[5]];
        let code_len = u32::from_le_bytes(code_len_bytes) as usize;

        let args_len_bytes = [slice[6], slice[7], slice[8], slice[9]];
        let args_len = u32::from_le_bytes(args_len_bytes) as usize;

        let data_len_bytes = [slice[10], slice[11], slice[12], slice[13]];
        let data_len = u32::from_le_bytes(data_len_bytes) as usize;

        let code = &slice[14..(14 + code_len)];
        let args = &slice[(14 + code_len)..(14 + code_len + args_len)];
        let data = &slice[(14 + code_len + args_len)..(14 + code_len + args_len + data_len)];

        Ok(Self {
            version,
            script_type,
            code,
            args,
            data,
        })
    }

    pub fn address(&self) -> Address {
        let mut hasher = sha3::Sha3_256::new();

        hasher.update(&[self.version, self.script_type.to_u8()]);
        hasher.update(self.code);
        hasher.update(self.args);

        let hash = hasher.finalize();

        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[..20]);

        Address(address)
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn script_type(&self) -> &ScriptType {
        &self.script_type
    }

    pub fn code(&self) -> &'a [u8] {
        self.code
    }

    pub fn args(&self) -> &'a [u8] {
        self.args
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}
