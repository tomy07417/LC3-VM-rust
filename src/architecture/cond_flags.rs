/// LC-3 condition flags stored in the COND register.
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum CondFlag {
    Pos = 1 << 0, // P
    Zro = 1 << 1, // Z
    Neg = 1 << 2, // N
}

impl From<CondFlag> for u16 {
    fn from(flag: CondFlag) -> Self {
        flag as u16
    }
}
