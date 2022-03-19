extern crate minifb;

use std::process;
use std::env;
use std::io;
use std::io::Write;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::fmt::LowerHex;
use num::Integer;
use std::{thread, time};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::Rng;

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
    keys: u16 // bit field of which keys were pressed
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
            keys: 0,
        };

        load_program(&mut chip8, file_path);
        load_fonts(&mut chip8);

        chip8
    }

    fn decrement_timers(&mut self) {
        if self.delay_t > 0 {
            self.delay_t -= 1;
        }

        if self.sound_t > 0 {
            self.sound_t -= 1;
        }
    }

    fn get_instr(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize+1] as u16
    }

    fn reset_keys(&mut self) {
        self.keys = 0;
    }

    fn register_keypresses(&mut self, keys: Vec<Key>) {
        self.keys = keys.iter().map(|key| {
            let exponent = match key {
                Key::Key1 => Some(0x1),
                Key::Key2 => Some(0x2),
                Key::Key3 => Some(0x3),
                Key::Key4 => Some(0xC),
                Key::Q =>    Some(0x4),
                Key::W =>    Some(0x5),
                Key::E =>    Some(0x6),
                Key::R =>    Some(0xD),
                Key::A =>    Some(0x7),
                Key::S =>    Some(0x8),
                Key::D =>    Some(0x9),
                Key::F =>    Some(0xE),
                Key::Z =>    Some(0xA),
                Key::X =>    Some(0x0),
                Key::C =>    Some(0xB),
                Key::V =>    Some(0xF),
                _ => None
            };

            if let Some(exp) = exponent {
                return 1 << exp; 
            } else {
                return 0;
            }
        })
        .sum::<u16>();
    }

    fn emulate_cycle(&mut self) {
        let instr = self.get_instr();
        // println!("{:#x}", instr);

        let opcode: u16 = instr & 0xF000;
        self.pc += 2;

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
                        self.display.fill(0);
                    }
                    0x0EE => {
                        // return from subroutine
                        self.pc = self.stack.pop().unwrap();
                    }
                    _ => ()
                }
            }
            0x1000 => {
                // jump
                self.pc = nnn;
            }
            0x2000 => {
                // subroutine call
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            0x3000 => {
                // skip if eq
                let vx = self.registers[x];
                if vx == nn { self.pc += 2; }
            }
            0x4000 => {
                // skip if neq
                let vx = self.registers[x];
                if vx != nn { self.pc += 2; }
            }
            0x5000 => {
                let vx = self.registers[x];
                let vy = self.registers[y];
                if vx == vy { self.pc += 2; }
            }
            0x6000 => {
                // set register vx to nn
                self.registers[x] = nn;
            }
            0x7000 => {
                // register ops 
                self.registers[x] = self.registers[x].overflowing_add(get_nn(instr)).0;
            }
            0x8000 => {
                // arithmetic
                match n {
                    0 => {
                        // set vx to vy 
                        self.registers[x] = self.registers[y];
                    }
                    1 => {
                        self.registers[x] |= self.registers[y];
                    }
                    2 => {
                        self.registers[x] &= self.registers[y];
                    }
                    3 => {
                        self.registers[x] ^= self.registers[y];
                    }
                    4 => {
                        let sum: u16 = self.registers[x] as u16 + self.registers[y] as u16;
                        self.registers[x] = (sum & 0x00FF) as u8; // only last 8 bits
                        self.registers[0xF] = if sum > 255 {1} else {0}; // check for overflow;
                    }
                    5 | 7 => {
                        let (left, right): (u8, u8) = match n {
                            5 => (self.registers[x], self.registers[y]),
                            7 => (self.registers[y], self.registers[x]),
                            _ => unreachable!(),
                        };
                        let (diff, underflow) = left.overflowing_sub(right);
                        self.registers[x] = diff;
                        self.registers[0xF] = if underflow {0} else {1};
                    }
                    6 | 0xE => {
                        let (new_val, flag_set) = match n {
                            6 =>   (self.registers[x] >> 1, self.registers[x] & 0x1),
                            0xE => (self.registers[x] << 1, (self.registers[x] >> 7) & 0x1),
                            _ => unreachable!(),
                        };
                        self.registers[x] = new_val;
                        self.registers[0xF] = flag_set; 
                    }
                    _ => unreachable!()
                        
                }
            }
            0x9000 => {
                let vx = self.registers[x];
                let vy = self.registers[y];
                if vx != vy { self.pc += 2; }
            }
            0xA000 => {
                // set index register
                self.ir = nnn;
            }
            0xB000 => {
                // jmp w/ offset (following impl of COSMAC VIP interpreter)
                // TODO: support toggle to behavior where self.registers[x] is used
                self.pc = nnn + self.registers[0] as u16;
            }
            0xC000 => {
                // random num AND -> vx
                self.registers[x] = nn & rand::thread_rng().gen_range(0..=255);
            }
            0xD000 => {
                // display to screen
                let sprite_height = n as usize;
                let vx = self.registers[x] as usize; 
                let vy = self.registers[y] as usize;
                let mut collide_flag: u8 = 0;

                for row in 0..sprite_height {

                    let mut sprite: u8 = self.memory[(self.ir + row as u16) as usize];

                    for col in (0..8).rev() {
                        let vx_w = (vx + col) % DISPLAY_WIDTH; 
                        let vy_w = (vy + row) % DISPLAY_HEIGHT;

                        let i = DISPLAY_WIDTH * (vy_w) + (vx_w);
                        if (self.display[i] == 0x1) && (sprite & 0x1) == 0x1 {
                            collide_flag = 1;
                        }
                        self.display[i] ^= sprite & 0x1; 
                        sprite >>= 1;
                    }
                }

                self.registers[0xF] = collide_flag;
            }
            0xE000 => {
                // skip if key press
                let skip_if = match nn {
                    0x9E => 1,
                    0xA1 => 0,
                    _ => unreachable!()
                };

                if (self.keys & 1 << self.registers[x]) == skip_if {
                    self.pc += 2;
                }
            }
            0xF000 => {
                match nn {
                    0x07 => {
                        self.registers[x] = self.delay_t;
                    }
                    0x0A => {
                        if self.keys == 0 {
                            // no keys are being pressed in this cycle, block
                            self.pc -= 2; 
                        } else {
                            // at least 1 key pressed
                            let mut i: u8 = 0;
                            let mut val = self.keys;
                            while val > 0 {
                                if (val & 0x1) == 1 { break; }
                                val >>= 1;
                                i += 1;
                            }
                            self.registers[x] = i;
                        }
                    }
                    0x15 => {
                        self.delay_t = self.registers[x];
                    }
                    0x18 => {
                        self.sound_t = self.registers[x];
                    }
                    0x1E => {
                        // TODO: Spaceflight 209! relies on overflow to cause VF=1
                        self.ir += self.registers[x] as u16; 
                    }
                    0x29 => {
                        // point to font character
                        self.ir = FONT_START_ADDR as u16 + (FONT_HEIGHT * self.registers[x]) as u16;
                    }
                    0x33 => {
                        // binary coded decimal conv
                        let digit: u8 = self.registers[x];
                        self.memory[(self.ir) as usize] = (digit / 100) % 10;
                        self.memory[(self.ir+1) as usize] = (digit / 10) % 10;
                        self.memory[(self.ir+2) as usize] = digit  % 10;
                    }
                    0x55 => {
                        let i = self.ir as usize;
                        self.memory[i..=i+x as usize].copy_from_slice(&self.registers[0..=x as usize]);
                    }
                    0x65 => {
                        let i = self.ir as usize;
                        self.registers[0..=x as usize].copy_from_slice(&self.memory[i..=i+x as usize]);
                    }
                    _ => { unreachable!(); }
                }
            }
            _ =>  { panic!("Unknown code {:#x}", opcode); }
        }
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

#[inline]
fn print_line_debug<T: Integer + LowerHex>(name: &str, value: T) {
    println!("{:<5}  {:#x}", format!("{}:", name), value);
}

fn print_lines_debug(chip: &Chip8) {
    print_line_debug("PC", chip.pc);
    print_line_debug("I", chip.ir);
    println!();
    for i in 0..=0xf {
        print_line_debug(format!("V{i}").as_ref(), chip.registers[i]);
    }
    println!();
    print_line_debug("ST", chip.sound_t);
    print_line_debug("DT", chip.delay_t);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // does not contain path to .ch8 program  
        eprintln!("Error: Missing path to CHIP-8 program to emulate");
        eprintln!("USAGE: cargo run <file_path>");
        process::exit(1);
    }

    let mut debug = false;

    if args.len() == 3 && args[2] == "-d" {
        // enter debug mode
        debug = true;

        println!("Starting program in debug mode...");
        println!("USAGE: ");
        println!("  r        - run until next checkpoint");
        println!("  b <addr> - add breakpoint at address <addr>");
        println!("  ni       - execute next instruction");
        println!("  p        - print current state of CHIP-8");
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

    let mut cycles_elapsed_since_timer = 0;

    // debugging stuff
    let mut breakpoints: HashSet<u16> = HashSet::new();

    // start fetching
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        // check if current pc is in breakpoints to pause at
        if breakpoints.contains(&chip.pc) {
            println!("Hit a breakpoint at {:#x}", chip.pc);
            debug = true;
        }

        if debug {
            let stdin = io::stdin(); 
            let input = &mut String::new();

            loop {
                print!(">> ");
                let _ = io::stdout().flush();
                input.clear();
                let _ = stdin.read_line(input); // blocks
                let commands: Vec<&str> = input.split_whitespace().collect();

                match commands[0] {
                    "r" =>  {
                        // run until we hit a breakpoint, or run until end
                        debug = false;
                        break;
                    }
                    "b" => {
                        let breakpoint: u16;

                        if commands.len() == 1 {
                            eprintln!("No breakpoint specified.");
                            eprintln!("Usage: b <address>");
                        }

                        match u16::from_str_radix(commands[1], 16) {
                            Ok(addr) => {
                                breakpoint = addr;
                            }
                            Err(_) => {
                                eprintln!("Badly formatted hex address.");
                                eprintln!("Enter a valid hex address without leading '0x'");
                                continue;
                            }
                        }
                        // set a breakpoint at <breakpoint> 
                        println!("Setting break point at {:#x}", breakpoint);
                        breakpoints.insert(breakpoint);
                    }
                    "p" => {
                        // print debugging information
                        print_lines_debug(&chip);
                    }
                    "ni" => {
                        // next instruction
                        println!("{:#x}\topcode={:#x}", chip.pc, chip.get_instr());
                        break;
                    }
                    _ => { }
                }
            }
        }


        cycles_elapsed_since_timer += 1;

        chip.register_keypresses(
            window.get_keys_pressed(KeyRepeat::Yes)
        );

        chip.emulate_cycle();

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

        if cycles_elapsed_since_timer == 8 {
            // enforce ~62.5Hz timer updates 
            chip.decrement_timers();
            cycles_elapsed_since_timer = 0;
        }

        chip.reset_keys();

        window
            .update_with_buffer(&buffer, win_width, win_height)
            .unwrap();
        
        // enforce ~500 for main loop
        // 500Hz -> 1/500s per cycle -> execute every 2ms
        thread::sleep(time::Duration::from_millis(2));
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
        chip.emulate_cycle();
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
        chip.emulate_cycle();
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
        chip.emulate_cycle();
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
        chip.emulate_cycle();
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
        chip.emulate_cycle();
        assert_eq!(chip.registers[1], 0xe8);
        assert_eq!(chip.registers[0xF], 0);
    } 

    #[test]
    fn keypress_detected() {
        let file_path = "programs/test_opcode.ch8";
        let mut chip = Chip8::init(file_path);
        chip.register_keypresses(vec![Key::Key1, Key::W, Key::Q, Key::V]);
        assert_eq!(chip.keys, 0b1000000000110010);
    } 
}
