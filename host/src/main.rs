use anyhow::Result;
// use wasi_common::sync::WasiCtxBuilder;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

// Generated wit code does not follow rust conventions completely
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod binding;

use binding::{HelloWorld, MyState};
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;

    let mut wasm_path = std::env::current_exe()?;
    wasm_path.pop();
    wasm_path.push("game.wasm");

    let component = Component::from_file(&engine, wasm_path)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    HelloWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    let mut wasi = WasiCtxBuilder::new();

    let mut store = Store::new(
        &engine,
        MyState {
            ctx: wasi.build(),
            table: ResourceTable::new(),
        },
    );
    let (bindings, _) = HelloWorld::instantiate(&mut store, &component, &linker)?;

    bindings.call_run(&mut store)?;
    Ok(())
}
