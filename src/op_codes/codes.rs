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

#[cfg(test)]
mod tests {
    use super::OpCode;

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
}
