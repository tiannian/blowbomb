use std::path::Path;

use anyhow::Result;
use bbm_primitives::{LeafId, Script, UnsignedTransaction};
use wasmtime::{Cache, CacheConfig, Config, Engine};

use crate::{WasmExecutorConfig, executors::WasmInstance};

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

    pub fn validate_script(
        &self,
        script: &Script<'_>,
        transaction: &UnsignedTransaction,
        unlocker: Vec<u8>,
    ) -> Result<()> {
        let mut instance = WasmInstance::new(
            &self.engine,
            script.code().to_vec(),
            Some(script.args().to_vec()),
            &transaction,
            Some(unlocker),
        )?;

        instance.run()?;

        Ok(())
    }

    pub fn validate_operator(
        &self,
        operator: Vec<u8>,
        // TODO: use leaf id to cache
        _operator_leaf_id: LeafId,
        transaction: &UnsignedTransaction,
    ) -> Result<()> {
        let mut instance = WasmInstance::new(&self.engine, operator, None, &transaction, None)?;

        instance.run()?;

        Ok(())
    }
}
