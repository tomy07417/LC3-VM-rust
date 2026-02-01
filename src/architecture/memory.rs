
const MEMORY_SIZE: usize = 1 << 16;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn read_u16(&self, address: u16) -> u16 {
        let addr = address as usize;
        let high_byte = self.data[addr] as u16;
        let low_byte = self.data[addr + 1] as u16;
        (high_byte << 8) | low_byte
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let addr = address as usize;
        self.data[addr] = (value >> 8) as u8;
        self.data[addr + 1] = (value & 0xFF) as u8;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: [0; MEMORY_SIZE],
        }
    }
}