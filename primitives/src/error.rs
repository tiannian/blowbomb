#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("wrong length {0} for unlock script, expected {1}")]
    WrongLengthForUnlockScript(usize, usize),

    #[error("wrong length {0} for tx, expected {1}")]
    WrongLengthForTx(usize, usize),

    #[error("wrong length {0} for fixed bytes, expected {1}")]
    WrongLengthForFixedBytes(usize, usize),

    // #[error("input and unlocker length mismatch, input length: {0}, unlocker length: {1}")]
    // InputUnlockerLengthMismatch(usize, usize),
    #[error("wrong length {0} for leaf, expected {1}")]
    WrongLengthForLeaf(usize, usize),
}

pub type Result<T> = core::result::Result<T, Error>;
