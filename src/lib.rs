pub mod vm;
pub mod display;
pub mod memory;
pub mod keypad;

#[cfg(target_arch="wasm32")]
pub mod wasm;
