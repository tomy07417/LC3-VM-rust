/// LC-3 TRAP vector codes (lower 8 bits of a TRAP instruction).
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapCode {
    Getc = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    Out = 0x21,   /* output a character */
    Puts = 0x22,  /* output a word string */
    In = 0x23,    /* get character from keyboard, echoed onto the terminal */
    Putsp = 0x24, /* output a byte string */
    Halt = 0x25,  /* halt the program */
}

impl PartialEq<u16> for TrapCode {
    fn eq(&self, other: &u16) -> bool {
        *self as u16 == *other
    }
}

impl From<u16> for TrapCode {
    fn from(value: u16) -> Self {
        match value {
            0x20 => TrapCode::Getc,
            0x21 => TrapCode::Out,
            0x22 => TrapCode::Puts,
            0x23 => TrapCode::In,
            0x24 => TrapCode::Putsp,
            0x25 => TrapCode::Halt,
            _ => panic!("Invalid trap code value: {}", value),
        }
    }
}
