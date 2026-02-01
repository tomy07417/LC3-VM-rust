use crate::architecture::registers::Register;

const MEMORY_SIZE: usize = 1 << 16;
pub struct VM {
    reg: [u16; Register::COUNT as usize], // Registers including PC and COND
    memory: [u8; MEMORY_SIZE],            // Memory
}