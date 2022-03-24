import { WasmVM } from "chip8";

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

const load_rom = async () => {
    const res = await fetch("./programs/space_invaders.ch8");
    const bytes = await res.arrayBuffer();
    return new Uint8Array(bytes, 0, bytes.byteLength);
}

const run = async () => {
    const vm = new WasmVM();
    const rom = await load_rom(); 
    vm.load_program(rom);

    document.addEventListener("keydown", (e) => {
        vm.set_key(e.code, true);
    })

    document.addEventListener("keyup", (e) => {
        vm.set_key(e.code, false);
    })

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');
    ctx.fillStyle = 'white';

    cycle_loop(vm);
}

run();

