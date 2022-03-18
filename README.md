# Chip-8 Emulator in Rust

### Running the emulator
```bash
cargo run <file_path>
```

`file_path` is the path to the `.ch8` file containing the opcodes.

Currently, the `programs` directory contains some test ROMs:
- `test_opcode.ch8` ([Source](https://github.com/corax89/chip8-test-rom))
- `ibm_logo.ch8` ([Source](https://github.com/loktar00/chip8/blob/master/roms/IBM%20Logo.ch8))
- `tetris.ch8` ([Source](https://github.com/dmatlack/chip8))

### Running tests
To execute tests for the emulator, run `cargo test`

