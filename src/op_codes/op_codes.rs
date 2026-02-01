pub enum OpCode {
    OpBr,   /* branch */
    OpAdd,  /* add  */
    OpLd,   /* load */
    OpSt,   /* store */
    OpJsr,  /* jump register */
    OpAnd,  /* bitwise and */
    OpLdr,  /* load register */
    OpStr,  /* store register */
    OpRti,  /* unused */
    OpNot,  /* bitwise not */
    OpLdi,  /* load indirect */
    OpSti,  /* store indirect */
    OpJmp,  /* jump */
    OpRes,  /* reserved (unused) */
    OpLea,  /* load effective address */
    OpTrap, /* execute trap */
}

#[cfg(test)]
mod tests {
    use super::OpCode;

    #[test]
    fn opcode_ordinals_match_lc3_order() {
        assert_eq!(OpCode::OpBr as u16, 0x0);
        assert_eq!(OpCode::OpAdd as u16, 0x1);
        assert_eq!(OpCode::OpLd as u16, 0x2);
        assert_eq!(OpCode::OpSt as u16, 0x3);
        assert_eq!(OpCode::OpJsr as u16, 0x4);
        assert_eq!(OpCode::OpAnd as u16, 0x5);
        assert_eq!(OpCode::OpLdr as u16, 0x6);
        assert_eq!(OpCode::OpStr as u16, 0x7);
        assert_eq!(OpCode::OpRti as u16, 0x8);
        assert_eq!(OpCode::OpNot as u16, 0x9);
        assert_eq!(OpCode::OpLdi as u16, 0xA);
        assert_eq!(OpCode::OpSti as u16, 0xB);
        assert_eq!(OpCode::OpJmp as u16, 0xC);
        assert_eq!(OpCode::OpRes as u16, 0xD);
        assert_eq!(OpCode::OpLea as u16, 0xE);
        assert_eq!(OpCode::OpTrap as u16, 0xF);
    }
}
