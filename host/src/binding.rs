use component::bindgen;

use wasmtime::*;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};

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
