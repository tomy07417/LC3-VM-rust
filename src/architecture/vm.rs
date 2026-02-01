use crate::architecture::Register;

const MEMORY_SIZE: usize = 1 << 16;

/// LC-3 virtual machine state (registers, memory, and run flag).
pub struct VM {
    reg: [u16; Register::Count as usize], // Registers including PC and COND
    memory: [u16; MEMORY_SIZE],           // Memory
    running: bool,
}

impl VM {
    /// Create a new VM with zeroed memory and registers.
    ///
    /// The program counter starts at the LC-3 default of `0x3000` and the VM
    /// is marked as running.
    pub fn new() -> Self {
        let mut reg = [0; Register::Count as usize];
        reg[Register::PC as usize] = 0x3000; // Initialize PC

        Self {
            reg,
            memory: [0; MEMORY_SIZE],
            running: true,
        }
    }

    /// Returns whether the VM is still running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Stop the VM execution loop.
    pub fn shut_down(&mut self) {
        self.running = false;
    }

    /// Read the value of a register by index.
    pub fn reg(&self, r: usize) -> u16 {
        self.reg[r]
    }

    /// Write a value to a register by index.
    pub fn set_reg(&mut self, r: usize, val: u16) {
        self.reg[r] = val;
    }

    /// Read a 16-bit value from memory.
    pub fn read_mem(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    /// Write a 16-bit value to memory.
    pub fn write_mem(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
