
#[derive(Debug, Clone, Copy)]
pub enum CondFlag {
    FlPos = 1 << 0, // P
    FlZro     = 1 << 1, // Z
    FlNeg = 1 << 2, // N
}