extern crate minifb;

use std::process;
use std::env;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use minifb::{Key, Window, WindowOptions};

const START_ADDR: usize = 0x200;
const FONT_START_ADDR: usize = 0x50; 
const FONT_END_ADDR: usize = 0xA0; 
const MEM_SIZE: usize = 4096; 
const DISPLAY_WIDTH: usize = 64; 
const DISPLAY_HEIGHT: usize = 32; 
const PX_SCALING: usize = 10;  // pixel scaling factor
const NUM_REGISTERS: usize = 16; 
const ON_PIXEL: u32 = 0x00FFFFFF; // white pixel
const FONT_HEIGHT: u8 = 5; // height (in pixels) that each digit of font occupies

static FONTS: [u8; 16 * FONT_HEIGHT as usize] = [
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
    let f = File::open(file_path).expect("Error: File not found");
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf).unwrap(); // todo: add error

    for (i, &byte) in buf.iter().enumerate() {
        chip8.memory[START_ADDR+i] = byte;
    }
}

fn get_x(instr: u16) -> usize {
    ((instr & 0x0F00) >> 8) as usize
}

fn get_y(instr: u16) -> usize {
    ((instr & 0x00F0) >> 4) as usize
}

fn get_n(instr: u16) -> u8 {
    (instr & 0x000F) as u8
}

fn get_nn(instr: u16) -> u8 {
    (instr & 0x00FF) as u8
}

fn get_nnn(instr: u16) -> u16 {
    instr & 0x0FFF
}

fn emulate_cycle(chip: &mut Chip8) {
    let pc: usize = chip.pc as usize;
    let instr: u16 = (chip.memory[pc] as u16) << 8 | chip.memory[pc+1] as u16;  
    // println!("{:#x}", instr);

    let opcode: u16 = instr & 0xF000;
    chip.pc += 2;

    let x = get_x(instr);
    let y = get_y(instr);
    let n = get_n(instr);
    let nn = get_nn(instr);
    let nnn = get_nnn(instr);

    match opcode {
        0x0 => { 
            match instr & 0x0FFF {
                0x0E0 => {
                    // clear display
                    chip.display.fill(0);
                }
                0x0EE => {
                    // return from subroutine
                    chip.pc = chip.stack.pop().unwrap();
                }
                _ => ()
            }
        }
        0x1000 => {
            // jump
            chip.pc = nnn;
        }
        0x2000 => {
            // subroutine call
            chip.stack.push(chip.pc);
            chip.pc = nnn;
        }
        0x3000 => {
            // skip if eq
            let vx = chip.registers[x];
            if vx == nn { chip.pc += 2; }
        }
        0x4000 => {
            // skip if neq
            let vx = chip.registers[x];
            if vx != nn { chip.pc += 2; }
        }
        0x5000 => {
            // skip if neq
            let vx = chip.registers[x];
            let vy = chip.registers[y];
            if vx == vy { chip.pc += 2; }
        }
        0x6000 => {
            // set register vx to nn
            chip.registers[x] = nn;
        }
        0x7000 => {
            // register ops 
            chip.registers[x] = chip.registers[x].overflowing_add(get_nn(instr)).0;
        }
        0x8000 => {
            // arithmetic
            match n {
                0 => {
                    // set vx to vy 
                    chip.registers[x] = chip.registers[y];
                }
                1 => {
                    chip.registers[x] |= chip.registers[y];
                }
                2 => {
                    chip.registers[x] &= chip.registers[y];
                }
                3 => {
                    chip.registers[x] ^= chip.registers[y];
                }
                4 => {
                    let sum: u16 = chip.registers[x] as u16 + chip.registers[y] as u16;
                    chip.registers[x] = (sum & 0x00FF) as u8; // only last 8 bits
                    chip.registers[0xF] = if sum > 255 {1} else {0}; // check for overflow;
                }
                5 | 7 => {
                    let (left, right): (u8, u8) = match n {
                        5 => (chip.registers[x], chip.registers[y]),
                        7 => (chip.registers[y], chip.registers[x]),
                        _ => unreachable!(),
                    };
                    let (diff, underflow) = left.overflowing_sub(right);
                    chip.registers[x] = diff;
                    chip.registers[0xF] = if underflow {0} else {1};
                }
                6 | 0xE => {
                    let (new_val, flag_set) = match n {
                        6 =>   (chip.registers[y] >> 1, chip.registers[y] & 0x1),
                        0xE => (chip.registers[y] << 1, (chip.registers[y] & 0x1 << 7) >> 7),
                        _ => unreachable!(),
                    };
                    chip.registers[x] = new_val;
                    chip.registers[0xF] = flag_set; 
                }
                _ => ()
                    
            }
        }
        0x9000 => {
            // set register vx 
            let vx = chip.registers[x];
            let vy = chip.registers[y];
            if vx != vy { chip.pc += 2; }
        }
        0xA000 => {
            // set index register
            chip.ir = nnn;
        }
        0xD000 => {
            // display to screen
            let sprite_height = n as u16;
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
        0xF000 => {
            match nn {
                0x07 => {
                    chip.registers[x] = chip.delay_t;
                }
                0x15 => {
                    chip.delay_t = chip.registers[x];
                }
                0x18 => {
                    chip.sound_t = chip.registers[x];
                }
                0x1E => {
                    // TODO: Spaceflight 209! relies on overflow to cause VF=1
                    chip.ir += chip.registers[x] as u16; 
                }
                0x0A => {
                    /*
                    if key not pressed {
                        chip.pc -= 2; // blocking execution from continuing
                    } else {
                        // key pressed
                        chip.registers[x] = hex value of key input
                    }
                    */
                }
                0x29 => {
                    // point to font character
                    chip.ir = FONT_START_ADDR as u16 + (FONT_HEIGHT * chip.registers[x]) as u16;
                }
                0x33 => {
                    // binary coded decimal conv
                    let digit: u8 = chip.registers[x];
                    chip.memory[(chip.ir) as usize] = (digit / 100) % 10;
                    chip.memory[(chip.ir+1) as usize] = (digit / 10) % 10;
                    chip.memory[(chip.ir+2) as usize] = digit  % 10;
                }
                0x55 => {
                    let i = chip.ir as usize;
                    chip.memory[i..=i+x as usize].copy_from_slice(&chip.registers[0..=x as usize]);
                }
                0x65 => {
                    let i = chip.ir as usize;
                    chip.registers[0..=x as usize].copy_from_slice(&chip.memory[i..=i+x as usize]);
                }
                _ => { unreachable!(); }
            }
        }
        _ =>  { unreachable!(); }
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

    let win_width = DISPLAY_WIDTH * PX_SCALING;
    let win_height = DISPLAY_HEIGHT * PX_SCALING;
    let mut buffer: Vec<u32> = vec![0; win_width * win_height];

    let mut window = Window::new (
        "CHIP-8",
        win_width,
        win_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // start fetching
    while window.is_open() && !window.is_key_down(Key::Escape) {
        emulate_cycle(&mut chip);

        for (i, px) in chip.display.into_iter().enumerate() {
            // px is u8, either 0x1 or 0x0
            let row = i / DISPLAY_WIDTH;
            let col = i % DISPLAY_WIDTH;

            for row_offset in 0..PX_SCALING  {
                let buf_idx = row*win_width*PX_SCALING + col*PX_SCALING + row_offset*win_width;
                buffer[buf_idx..buf_idx+PX_SCALING]
                    .fill(if px == 0 {0} else {ON_PIXEL});
            }
        }

        window
            .update_with_buffer(&buffer, win_width, win_height)
            .unwrap();
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

    #[test]
    fn lsb_shift_works_8xy6_shiftbit0() {
        let file_path = "programs/test_opcode.ch8";
        let mut chip = Chip8::init(file_path);
        chip.pc = 0;
        chip.registers[2] = 1; 
        chip.memory[0] = 0x81; // x = 1
        chip.memory[1] = 0x26; // y = 2
        emulate_cycle(&mut chip);
        assert_eq!(chip.registers[1], chip.registers[2] >> 1);
        assert_eq!(chip.registers[0xF], 1);
    } 

    #[test]
    fn lsb_shift_works_8xy6_shiftbit1() {
        let file_path = "programs/test_opcode.ch8";
        let mut chip = Chip8::init(file_path);
        chip.pc = 0;
        chip.registers[2] = 4; 
        chip.memory[0] = 0x81; // x = 1
        chip.memory[1] = 0x26; // y = 2
        emulate_cycle(&mut chip);
        assert_eq!(chip.registers[1], chip.registers[2] >> 1);
        assert_eq!(chip.registers[0xF], 0);
    } 

    #[test]
    fn msb_shift_works_8xye_shiftbit0() {
        let file_path = "programs/test_opcode.ch8";
        let mut chip = Chip8::init(file_path);
        chip.pc = 0;
        chip.registers[2] = 1; 
        chip.memory[0] = 0x81; // x = 1
        chip.memory[1] = 0x2E; // y = 2
        emulate_cycle(&mut chip);
        assert_eq!(chip.registers[1], chip.registers[2] << 1);
        assert_eq!(chip.registers[0xF], 0);
    } 

    #[test]
    fn msb_shift_works_8xye_shiftbit1() {
        let file_path = "programs/test_opcode.ch8";
        let mut chip = Chip8::init(file_path);
        chip.pc = 0;
        chip.registers[2] = 255; // 0b10000000
        chip.memory[0] = 0x81; // x = 1
        chip.memory[1] = 0x2E; // y = 2
        emulate_cycle(&mut chip);
        assert_eq!(chip.registers[1], chip.registers[2] << 1);
        assert_eq!(chip.registers[0xF], 1);
    } 

    #[test]
    fn subtract_underflow() {
        let file_path = "programs/test_opcode.ch8";
        let mut chip = Chip8::init(file_path);
        chip.pc = 0;
        chip.registers[1] = 0x08;  // register x
        chip.registers[2] = 0x20;  // register y
        chip.memory[0] = 0x81; // x = 1
        chip.memory[1] = 0x25; // y = 2
        emulate_cycle(&mut chip);
        assert_eq!(chip.registers[1], 0xe8);
        assert_eq!(chip.registers[0xF], 0);
    } 
}
