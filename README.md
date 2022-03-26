# Chip-8 Emulator in Rust

### Running the emulator for desktop
```bash
cargo run <file_path> [-d]
```
`file_path` is the path to the `.ch8` file containing the opcodes.
`-d` is an optional flag which enables debug mode:

Debug mode has several commands (type `help`):

```bash
USAGE: 
r        - run until next checkpoint
b <addr> - add breakpoint at address <addr>
ni       - execute next instruction
p        - print current state of CHIP-8
help     - print list of commands available
```

Currently, the `web/programs/` directory contains several ROMs and their descriptions from [here](https://github.com/kripod/chip8-roms).

### Building for WASM
Additionally, we allow compiling to the `wasm32-unknown-unknown` target with `wasm-bindgen`. First, get `wasm-pack` [here](https://rustwasm.github.io/wasm-pack/installer/). After that, to build wasm binaries, run:

```bash
wasm-pack build
```

### Running on browser
After generating the WASM binaries, go to `/web`, install depenencies with `npm i` and run:

```bash
npm run
```

Then go to `localhost:8080` to view the result.
