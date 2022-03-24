use crate::memory::Memory;
use crate::display::Display;
use crate::keypad::Keypad;

use std::fmt::LowerHex;
use num::Integer;
use rand::Rng;

const START_ADDR: usize = 0x200;
const FONT_START_ADDR: usize = 0x50; 
const FONT_END_ADDR: usize = 0xA0; 
const NUM_REGISTERS: usize = 16; 

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


pub struct VM {
    memory: Memory, // 4kb RAM
    pub pc: u16,                // program counter, 2^12 = 4096
    ir: u16,                // index register
    stack: Vec<u16>,        // should have 16 elements at any one time
    delay_t: u8,            // delay timer
    sound_t: u8,            // sound timer
    display: Display,       // display graphics
    registers: [u8; NUM_REGISTERS],   // 16 general-purpose registers
    keys: Keypad,
    pub redraw: bool,
}

impl VM {
    pub fn new() -> VM {
        let mut chip = VM {
            memory: Memory::new(),
            pc: START_ADDR as u16,
            ir: 0,
            stack: Vec::new(),
            delay_t: 0,
            sound_t: 0,
            display: Display::new(),
            registers: [0; NUM_REGISTERS],
            keys: Keypad::new(),
            redraw: false,
        };

        // load fonts
        chip.memory.map_range(FONT_START_ADDR, FONT_END_ADDR - FONT_START_ADDR, &FONTS);

        chip
    }

    pub fn register_keypress(&mut self, keys: Option<u8>) {
        if let Some(key) = keys {
            self.keys.set_key(key);
        }
    }

    pub fn load_program(&mut self, buf: &[u8]) {
        self.memory.map_range(START_ADDR, buf.len(), buf);
    }

    pub fn get_display(&self) -> &[u8] {
        return &self.display.pixels; 
    }

    pub fn decrement_timers(&mut self) -> bool {
        if self.delay_t > 0 {
            self.delay_t -= 1;
        }

        if self.sound_t > 0 {
            self.sound_t -= 1;
            return true;
        }  

        false
    }

    pub fn emulate_cycle(&mut self) {
        let instr = self.memory.get_instr(self.pc);
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
                self.handle_0x0(nnn);
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
                self.handle_0x8(x, y, n);
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
                // get memory[ir..ir+sprite_height]
                let sprite: &[u8] = self.memory.get_range(self.ir as usize, sprite_height);

                let collide_flag: u8 = self.display.draw(vx, vy, sprite_height, sprite);
                self.redraw = true;

                self.registers[0xF] = collide_flag;
            }
            0xE000 => {
                // skip if key press
                self.handle_0xe(x, nn);
            }
            0xF000 => {
                self.handle_0xf(x, nn);
            }
            _ =>  { panic!("Unknown code {:#x}", opcode); }
        }
    }

    fn handle_0x0(&mut self, nnn: u16) {
        match nnn {
            0x0E0 => {
                // clear display
                self.display.clear();
                self.redraw = true;
            }
            0x0EE => {
                // return from subroutine
                self.pc = self.stack.pop().unwrap();
            }
            _ => ()
        }
    }

    fn handle_0x8(&mut self, x: usize, y: usize, n: u8) {
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

    fn handle_0xe(&mut self, x: usize, nn: u8) {
        match nn {
            0x9E =>  {
                if self.keys.is_pressed(self.registers[x]) {
                    self.pc += 2;
                }
            }
            0xA1 =>  {
                if !self.keys.is_pressed(self.registers[x]) {
                    self.pc += 2;
                }
            }
            _ => unreachable!()
        }
    }

    fn handle_0xf(&mut self, x: usize, nn: u8) {
        match nn {
            0x07 => {
                self.registers[x] = self.delay_t;
            }
            0x0A => {
                if self.keys.no_keys_pressed() {
                    self.pc -= 2; 
                } else {
                    // at least 1 key pressed
                    self.registers[x] = self.keys.get_first_key_pressed();
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
                self.memory.set(self.ir, (digit/100) % 10);
                self.memory.set(self.ir+1, (digit/10) % 10);
                self.memory.set(self.ir+2, digit % 10);
            }
            0x55 => {
                let i = self.ir as usize;
                self.memory.map_range(i, x+1, &self.registers[0..=x as usize]);
            }
            0x65 => {
                let i = self.ir as usize;
                self.registers[0..=x as usize].copy_from_slice(self.memory.get_range(i, x+1));
            }
            _ => { unreachable!(); }
        }
    }
}

#[inline]
fn get_line_debug<T: Integer + LowerHex>(name: &str, value: T) -> String {
    format!("{:<5}  {:#x}", format!("{}:", name), value)
}

impl std::fmt::Debug for VM {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vm_debug_info: Vec<String> = Vec::new();
        vm_debug_info.push(get_line_debug("PC", self.pc));
        vm_debug_info.push(get_line_debug("I", self.ir));
        for i in 0..=0xf {
            vm_debug_info.push(get_line_debug(format!("V{i}").as_ref(), self.registers[i]));
        }
        vm_debug_info.push(get_line_debug("ST", self.sound_t));
        vm_debug_info.push(get_line_debug("DT", self.delay_t));
        fmt.write_str(&vm_debug_info.join("\n")[..]).unwrap();
        Ok(())
    }
}

