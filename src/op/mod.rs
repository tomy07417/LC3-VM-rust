pub mod load;
mod op_codes;
pub mod store;
pub mod trap;

pub use op_codes::{OpCode, decode_opcode};
