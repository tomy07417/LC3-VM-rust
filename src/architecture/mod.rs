mod cond_flags;
mod registers;
mod vm;

pub use cond_flags::CondFlag;
pub use registers::Register;
pub use vm::VM;
/// Default LC-3 program counter start address.
pub const START_PC: u16 = 0x3000;
