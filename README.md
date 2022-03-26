# Chip-8 Emulator in Rust

<p align="center">
    <img src="https://raw.githubusercontent.com/sarckk/rust-chip8/master/docs/screenshot_desktop.png" alt="screenshot" />
</p>


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

### Key mapping
```
      Chip-8                           Keyboard
+────+────+────+────+            +────+────+────+────+     
| 1  | 2  | 3  | C  |            | 1  | 2  | 3  | 4  |
+────+────+────+────+            +────+────+────+────+
| 4  | 5  | 6  | D  |            | Q  | W  | E  | R  |
+────+────+────+────+     -->    +────+────+────+────+    
| 7  | 8  | 9  | E  |            | A  | S  | D  | F  |
+────+────+────+────+            +────+────+────+────+
| A  | 0  | B  | F  |            | Z  | X  | C  | V  |
+────+────+────+────+            +────+────+────+────+
```

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

### Credits
The `/web` directory was bootstrapped with `npm init wasm-app`.

Here are some helpful resources for getting started on Chip8 and WASM:
- https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
- https://rustwasm.github.io/docs/book/
