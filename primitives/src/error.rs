#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("wrong length {0} for leaf id, expected 36")]
    WrongLengthForLeafId(usize),

    #[error("wrong length {0} for script, expected {1}")]
    WrongLengthForScript(usize, usize),

    #[error("wrong length {0} for fixed bytes, expected {1}")]
    WrongLengthForFixedBytes(usize, usize),

    #[error("wrong length {0} for leaf, expected {1}")]
    WrongLengthForLeaf(usize, usize),

    #[error("wrong length {0} for tx, expected {1}")]
    WrongLengthForTx(usize, usize),

    #[error("wrong length {0} for bytes, expected {1}")]
    WrongLengthForBytes(usize, usize),
}

pub type Result<T> = core::result::Result<T, Error>;
