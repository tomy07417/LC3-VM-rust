/// LC-3 instruction opcode (upper 4 bits of the instruction word).
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Br,   /* branch */
    Add,  /* add  */
    Ld,   /* load */
    St,   /* store */
    Jsr,  /* jump register */
    And,  /* bitwise and */
    Ldr,  /* load register */
    Str,  /* store register */
    Rti,  /* unused */
    Not,  /* bitwise not */
    Ldi,  /* load indirect */
    Sti,  /* store indirect */
    Jmp,  /* jump */
    Res,  /* reserved (unused) */
    Lea,  /* load effective address */
    Trap, /* execute trap */
}

/// Decode the opcode bits (bits 15..12) from an instruction word.
pub fn decode_opcode(instr: u16) -> u16 {
    instr >> 12
}

impl PartialEq<u16> for OpCode {
    fn eq(&self, other: &u16) -> bool {
        *self as u16 == *other
    }
}

impl From<u16> for OpCode {
    fn from(value: u16) -> Self {
        match value {
            0x0 => OpCode::Br,
            0x1 => OpCode::Add,
            0x2 => OpCode::Ld,
            0x3 => OpCode::St,
            0x4 => OpCode::Jsr,
            0x5 => OpCode::And,
            0x6 => OpCode::Ldr,
            0x7 => OpCode::Str,
            0x8 => OpCode::Rti,
            0x9 => OpCode::Not,
            0xA => OpCode::Ldi,
            0xB => OpCode::Sti,
            0xC => OpCode::Jmp,
            0xD => OpCode::Res,
            0xE => OpCode::Lea,
            0xF => OpCode::Trap,
            _ => panic!("Invalid opcode value: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OpCode;
    use super::decode_opcode;

    #[test]
    fn opcode_ordinals_match_lc3_order() {
        assert_eq!(OpCode::Br as u16, 0x0);
        assert_eq!(OpCode::Add as u16, 0x1);
        assert_eq!(OpCode::Ld as u16, 0x2);
        assert_eq!(OpCode::St as u16, 0x3);
        assert_eq!(OpCode::Jsr as u16, 0x4);
        assert_eq!(OpCode::And as u16, 0x5);
        assert_eq!(OpCode::Ldr as u16, 0x6);
        assert_eq!(OpCode::Str as u16, 0x7);
        assert_eq!(OpCode::Rti as u16, 0x8);
        assert_eq!(OpCode::Not as u16, 0x9);
        assert_eq!(OpCode::Ldi as u16, 0xA);
        assert_eq!(OpCode::Sti as u16, 0xB);
        assert_eq!(OpCode::Jmp as u16, 0xC);
        assert_eq!(OpCode::Res as u16, 0xD);
        assert_eq!(OpCode::Lea as u16, 0xE);
        assert_eq!(OpCode::Trap as u16, 0xF);
    }

    #[test]
    fn decode_opcode_extracts_upper_nibble() {
        for value in 0x0u16..=0xFu16 {
            let instr = (value << 12) | 0x00A;
            assert_eq!(decode_opcode(instr), value);
        }
    }
}
