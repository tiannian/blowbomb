use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

pub struct WasmExecutorConfig {
    pub config_path: PathBuf,
    pub cache_path: Option<PathBuf>,
}
