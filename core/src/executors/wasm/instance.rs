use anyhow::Result;
use bbm_primitives::UnsignedTransaction;
use wasmtime::{Engine, Instance, Module, Store};

pub(crate) struct ExecutorStore {
    pub unsigned: Vec<u8>,
    pub unlocker: Option<Vec<u8>>,
    pub args: Option<Vec<u8>>,
}

pub(crate) struct WasmInstance {
    instance: Instance,
    store: Store<ExecutorStore>,
}

impl WasmInstance {
    pub fn new(
        engine: &Engine,
        code: Vec<u8>,
        args: Option<Vec<u8>>,
        unsigned: &UnsignedTransaction,
        unlocker: Option<Vec<u8>>,
    ) -> Result<Self> {
        let unsigned = unsigned.to_vec()?;

        let mut store = Store::new(
            engine,
            ExecutorStore {
                unsigned,
                unlocker,
                args,
            },
        );

        let module = Module::from_binary(engine, &code)?;

        let imports = &[];

        let instance = Instance::new(&mut store, &module, imports)?;

        Ok(Self { instance, store })
    }

    pub fn run(&mut self) -> Result<()> {
        let func = self
            .instance
            .get_typed_func::<(), u32>(&mut self.store, "_entry")?;

        func.call(&mut self.store, ())?;

        Ok(())
    }
}
