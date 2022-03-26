import { WasmVM } from "chip8";
import test from "./roms.txt";
import Alpine from 'alpinejs';

const roms = test.split('\n');
const initRom = roms[0];

const cycle_loop = (vm) => {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // 600 fps
    let redraw = false;

    for(let i = 0; i < 10; i++) {
        vm.emulate_cycle();
        if(vm.should_redraw()) {
            redraw = true;
        }
    }

    // runs at 60fps
    let beep = vm.decrement_timers();
    if(beep) {
        // play a beep
    }

    if(redraw) {
        const display = vm.get_display();
        display.forEach((px, i) => {
            const row = Math.floor(i / 64);
            const col = i % 64;
            if(px == 1) {
                ctx.fillRect(col*10, row*10, 10, 10);
            } 
        });
    }


    window.requestAnimationFrame(() => {
        cycle_loop(vm);
    });
}

const load_rom = async (rom) => {
    const res = await fetch(`./programs/${rom}.ch8`);
    const res_instr = await fetch(`./programs/${rom}.txt`);
    const instructions = await res_instr.text(); 
    const bytes = await res.arrayBuffer();
    return [new Uint8Array(bytes, 0, bytes.byteLength), instructions];
}

const run = async (rom) => {
    const vm = new WasmVM();
    vm.load_program(rom);

    document.addEventListener("keydown", (e) => {
        vm.set_key(e.code, true);
    })

    document.addEventListener("keyup", (e) => {
        vm.set_key(e.code, false);
    })

    cycle_loop(vm);
}

document.addEventListener("alpine:init", async () => {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    ctx.fillStyle = '#e0e0e0';

    Alpine.data("chip8", () => ({
        roms,
        selectedRom: initRom,
        instructions: "",

        init() {
            this.runRom(initRom);
        },

        async runRom(romName) {
            const [rom, instructions] = await load_rom(romName); 
            this.instructions = instructions;
            console.log(this.instructions);
            run(rom);
        },

        uploadRom(event) {
            if(!event.target.files.length) return;
            this.instructions = "";

            let file = event.target.files[0];
            const reader = new FileReader();
            reader.readAsArrayBuffer(file);
            reader.onload = (e) => {
                const rom = new Uint8Array(reader.result);
                run(rom);
            }
        }
    }))
})

Alpine.start();





