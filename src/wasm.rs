extern crate console_error_panic_hook;

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

extern crate web_sys;

// macro from https://rustwasm.github.io/docs/book/game-of-life/debugging.html
macro_rules! log {
    ( $( $t:tt  )*  ) => {
            web_sys::console::log_1(&format!( $( $t  )*  ).into());
    }
}

#[wasm_bindgen]
impl WasmVM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmVM {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        WasmVM {
            vm: VM::new(),
        }
    }

    pub fn should_redraw(&self) -> bool {
        self.vm.redraw
    }

    pub fn set_key(&mut self, key: &str, pressed: bool) {
        // map key accordingly
        let key_mapped = match key {
            "Digit1" => Some(0x1),
            "Digit2" => Some(0x2),
            "Digit3" => Some(0x3),
            "Digit4" => Some(0x4),
            "KeyQ" =>    Some(0x4),
            "KeyW" =>    Some(0x5),
            "KeyE" =>    Some(0x6),
            "KeyR" =>    Some(0xD),
            "KeyA" =>    Some(0x7),
            "KeyS" =>    Some(0x8),
            "KeyD" =>    Some(0x9),
            "KeyF" =>    Some(0xE),
            "KeyZ" =>    Some(0xA),
            "KeyX" =>    Some(0x0),
            "KeyC" =>    Some(0xB),
            "KeyV" =>    Some(0xF),
            _ => None
        };


        if !key_mapped.is_none() {
            self.vm.set_key(key_mapped, pressed);
        }
    }

    pub fn load_program(&mut self, buf: &[u8]) {
        self.vm.load_program(buf);
    }

    pub fn emulate_cycle(&mut self) {
        self.vm.emulate_cycle();
    }

    pub fn decrement_timers(&mut self) -> bool {
        self.vm.decrement_timers()
    }

    pub fn get_display(&self) -> Uint8Array {
        let display = self.vm.get_display();
        Uint8Array::from(display)
    }
}

