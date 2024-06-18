use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use wasmtime::component::{bindgen, Component, Linker, ResourceAny};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use exports::example::host::game_api::{GuestGameInstance, RenderCommand};

bindgen!({
    path: "../wit"
});

pub struct MyState {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub struct WebAssemblyContext {
    store: Store<MyState>,
    engine: Engine,
}

impl WebAssemblyContext {
    pub fn load() -> Result<WebAssemblyContext> {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        let mut wasi = WasiCtxBuilder::new();

        let store = Store::new(
            &engine,
            MyState {
                ctx: wasi.build(),
                table: ResourceTable::new(),
            },
        );
        Ok(Self { store, engine })
    }
}

pub struct WebAssemblyInstance {
    bindings: HotreloadExample,
    context: Rc<RefCell<WebAssemblyContext>>,
}

impl WebAssemblyInstance {
    pub fn load(mut context: WebAssemblyContext) -> Result<WebAssemblyInstance> {
        let mut wasm_path = std::env::current_exe()?;
        wasm_path.pop();
        wasm_path.push("game.wasm");

        let component = Component::from_file(&context.engine, wasm_path)?;

        let mut linker = Linker::new(&context.engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;

        let (bindings, _) = HotreloadExample::instantiate(&mut context.store, &component, &linker)?;
        Ok(Self {
            bindings,
            context: Rc::new(RefCell::new(context)),
        })
    }

    pub fn create_game_instance(&mut self) -> Result<GameInstance> {
        let instance_type = self.bindings.example_host_game_api().game_instance();

        let instance = {
            let mut context = self.context.borrow_mut();
            instance_type.call_constructor(&mut context.store)?
        };

        Ok(GameInstance {
            instance_type,
            instance,
            context: self.context.clone(),
        })
    }
}

pub struct GameInstance<'a> {
    instance_type: GuestGameInstance<'a>,
    instance: ResourceAny,
    context: Rc<RefCell<WebAssemblyContext>>,
}

impl<'a> GameInstance<'a> {
    pub fn run_frame(&self) -> Result<Vec<RenderCommand>> {
        let mut context = self.context.borrow_mut();
        self.instance_type
            .call_run_frame(&mut context.store, self.instance)
    }
}
