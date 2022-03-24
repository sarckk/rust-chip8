use wasm_bindgen::prelude::*;
use crate::vm::VM;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WasmVM {
    vm: VM,
}

#[wasm_bindgen]
impl WasmVM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmVM {
        WasmVM {
            vm: VM::new(),
        }
    }
}

