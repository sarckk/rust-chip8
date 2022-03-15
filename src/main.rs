use std::process;
use std::env;
use std::io::Read;
use std::io::BufReader;
use std::fs;
use std::fs::File;

const START_ADDR: usize = 0x200;
const FONT_START_ADDR: usize = 0x50; 
const FONT_END_ADDR: usize = 0xA0; 
const MEM_SIZE: usize = 4096; 
const DISPLAY_WIDTH: usize = 64; 
const DISPLAY_HEIGHT: usize = 32; 
const NUM_REGISTERS: usize = 16; 

static FONTS: [u8; 5*16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    memory: [u8; MEM_SIZE], // 4kb RAM
    pc: u16,                // program counter, 2^12 = 4096
    ir: u16,                // index register
    stack: Vec<u16>,        // should have 16 elements at any one time
    delay_t: u8,            // delay timer
    sound_t: u8,            // sound timer
    display: [u8; DISPLAY_WIDTH*DISPLAY_HEIGHT], // display graphics
    registers: [u8; NUM_REGISTERS],   // 16 general-purpose registers
}

impl Chip8 {
    fn init(file_path: &str) -> Chip8 {
        let mut chip8 = Chip8 {
            memory: [0; MEM_SIZE],
            pc: START_ADDR as u16,
            ir: 0,
            stack: Vec::new(),
            delay_t: 0,
            sound_t: 0,
            display: [0; DISPLAY_WIDTH*DISPLAY_HEIGHT],
            registers: [0; NUM_REGISTERS],
        };

        load_program(&mut chip8, file_path);
        load_fonts(&mut chip8);

        chip8
    }
}

pub fn load_fonts(chip8: &mut Chip8) {
    chip8.memory[FONT_START_ADDR..FONT_END_ADDR].copy_from_slice(&FONTS);
}

pub fn load_program(chip8: &mut Chip8, file_path: &str) {
    let mut f = File::open(file_path).expect("Error: File not found");
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf).unwrap(); // todo: add error

    for (i, &byte) in buf.iter().enumerate() {
        chip8.memory[START_ADDR+i] = byte;
    }
}

fn is_clear(instr: u16) -> bool {
    (instr & 0x0FFF) == 0x0E0 
}

fn emulate_cycle(chip: &mut Chip8) {
    let pc: usize = chip.pc as usize;
    let instr: u16 = (chip.memory[pc] as u16) << 8 | chip.memory[pc+1] as u16;  
    // println!("{:#x}", instr);

    let opcode: u16 = instr & 0xF000;
    chip.pc += 2;

    match opcode {
        0x0 => { 
            if is_clear(instr) {
                // clear the screen
                chip.display.fill(0);
            }
        }
        0x1000 => {
            // jump
            // println!("setting pc to {:#x}", (instr & 0x0FFF));
            chip.pc = instr & 0x0FFF;
        }
        0x6000 => {
            // set register vx 
            let reg_idx: usize = ((instr & 0x0F00) >> 8) as usize;
            chip.registers[reg_idx] = (instr & 0x00FF) as u8;
        }
        0x7000 => {
            // register ops 
            let reg_idx: usize = ((instr & 0x0F00) >> 8) as usize;
            chip.registers[reg_idx] += (instr & 0x00FF) as u8;
        }
        0xa000 => {
            // set index register
            chip.ir = instr & 0x0FFF;
        }
        0xd000 => {
            // display to screen
            let x: usize = ((instr & 0x0F00) >> 8) as usize;
            let y: usize = ((instr & 0x00F0) >> 4) as usize;
            let sprite_height = instr & 0x000F;
            let vx = (chip.registers[x] as usize) & 63; // modulo
            let vy = (chip.registers[y] as usize) & 31;
            let mut collide_flag: u8 = 0;

            for row in 0..sprite_height {
                if vy + row as usize == DISPLAY_HEIGHT as usize {
                    break;
                }

                let mut sprite: u8 = chip.memory[(chip.ir + row) as usize];
                for col in (0..8).rev() {
                    if vx + col == DISPLAY_WIDTH {
                        break;
                    }

                    let i = DISPLAY_WIDTH * (vy + row as usize) + (vx + col as usize);
                    if (chip.display[i] == 0x1) && (sprite & 0x1) == 0x1 {
                        collide_flag = 1;
                    }
                    chip.display[i] |= sprite & 0x1; 
                    sprite >>= 1;
                }
            }

            chip.registers[0xF] = collide_flag;
        }
        _ =>  { 
            panic!("Encountered an unknown code {:#x}", opcode);
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // does not contain path to .ch8 program  
        eprintln!("Error: Missing path to CHIP-8 program to emulate");
        eprintln!("USAGE: cargo run <file_path>");
        process::exit(1);
    }

    let file_path = &args[1];
    let mut chip = Chip8::init(file_path);

    // start fetching
    while true {
        emulate_cycle(&mut chip);
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn load_program_correctly() {
        let file_path = "programs/test_opcode.ch8";
        let chip = Chip8::init(file_path);
        assert_eq!(chip.memory[START_ADDR], 0x12);
        assert_eq!(chip.memory[START_ADDR+1], 0x4e);
    } 

    #[test]
    fn load_fonts_correctly() {
        let file_path = "programs/test_opcode.ch8";
        let chip = Chip8::init(file_path);
        assert_eq!(chip.memory[FONT_START_ADDR..FONT_END_ADDR], FONTS[..]);
    } 
}
