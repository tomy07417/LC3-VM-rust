pub mod load;
mod op_codes;
pub mod store;
pub mod trap;
pub mod alu;
pub mod jump;

pub use op_codes::{decode_opcode, OpCode};
