use crate::architecture::Register;

const MEMORY_SIZE: usize = 1 << 16;
pub struct VM {
    pub reg: [u16; Register::Count as usize], // Registers including PC and COND
    pub memory: [u16; MEMORY_SIZE],            // Memory
}

impl VM {
    pub fn new() -> Self {
        VM {
            reg: [0; Register::Count as usize],
            memory: [0; MEMORY_SIZE],
        }
    }

    #[inline]
    pub fn reg(&self, r: Register) -> u16 {
        self.reg[r as usize]
    }

    #[inline]
    pub fn set_reg(&mut self, r: Register, val: u16) {
        self.reg[r as usize] = val;
    }
}