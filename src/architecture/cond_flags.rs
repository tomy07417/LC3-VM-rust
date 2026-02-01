#[derive(Debug, Clone, Copy)]
pub enum CondFlag {
    Pos = 1 << 0, // P
    Zro = 1 << 1, // Z
    Neg = 1 << 2, // N
}
