#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("wrong length {0} for leaf id, expected 36")]
    WrongLengthForLeafId(usize),

    #[error("wrong length {0} for script, expected {1}")]
    WrongLengthForScript(usize, usize),
}

pub type Result<T> = core::result::Result<T, Error>;
