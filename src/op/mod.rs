mod op_codes;
mod trap_codes;
mod trap_op;

pub use op_codes::{OpCode, decode_opcode};
pub use trap_codes::TrapCode;
pub use trap_op::trap;
