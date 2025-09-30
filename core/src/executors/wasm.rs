use std::{collections::BTreeMap, path::Path};

use anyhow::Result;
use bbm_primitives::{Bytes, LeafId, Script, Transaction};
use wasmtime::{Cache, CacheConfig, Config, Engine};

use crate::WasmExecutorConfig;

pub struct WasmExecutor {
    engine: Engine,
}

impl WasmExecutor {
    pub fn new(config: &WasmExecutorConfig, home_path: &Path) -> Result<Self> {
        let mut cache_config = CacheConfig::from_file(Some(&config.config_path))?;
        if let Some(cache_path) = config.cache_path.clone() {
            cache_config.with_directory(cache_path);
        } else {
            cache_config.with_directory(home_path.join("cache"));
        }

        let cache = Cache::new(cache_config)?;

        let mut config = Config::new();
        config.cache(Some(cache));

        let engine = Engine::new(&config)?;

        Ok(Self { engine })
    }

    pub fn validate_script(&self, script: &Script<'_>, transaction: &Transaction) -> Result<()> {
        Ok(())
    }

    pub fn validate_operator(&self, operator: &Bytes, transaction: &Transaction) -> Result<()> {
        Ok(())
    }
}
