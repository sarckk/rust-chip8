use wasm_bindgen::prelude::*;
use crate::vm::VM;

use js_sys::{Uint8Array};

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

    pub fn should_redraw(&self) -> bool {
        self.vm.redraw
    }

    pub fn register_keypress(&mut self, key: u8) {
        self.vm.register_keypress(Some(key));
    }

    pub fn load_program(&mut self, buf: &[u8]) {
        self.vm.load_program(buf);
    }

    pub fn emulate_cycle(&mut self) {
        self.vm.emulate_cycle();
    }

    pub fn decrement_timers(&mut self) {
        self.vm.decrement_timers();
    }

    pub fn get_display(&self) -> Uint8Array {
        let display = self.vm.get_display();
        Uint8Array::from(display)
    }
}

