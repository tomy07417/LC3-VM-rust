
#[derive(Copy, Clone, Debug)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,   // Program Counter
    COND, // Condition Flags
    COUNT
}