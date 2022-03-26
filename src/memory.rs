const MEM_SIZE: usize = 4096; 

pub struct Memory {
    memory: [u8; MEM_SIZE], // 4kb RAM
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEM_SIZE]
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.memory.as_ptr()
    }

    pub fn get_instr(&self, pc: u16) -> u16 {
        (self.memory[pc as usize] as u16) << 8 | self.memory[pc as usize+1] as u16
    }

    pub fn map_range(&mut self, start: usize, size: usize, target: &[u8]) {
        self.memory[start..start+size].copy_from_slice(target);
    }

    pub fn get_range(&self, start: usize, size: usize) -> &[u8] {
        &self.memory[start..start+size]
    }

    // set single byte at specific address
    pub fn set(&mut self, index: u16, data: u8) {
        self.memory[index as usize] = data;
    }

    // get single byte at specific address
    pub fn get(&self, index: u16) -> u8 {
        self.memory[index as usize]
    }
}




