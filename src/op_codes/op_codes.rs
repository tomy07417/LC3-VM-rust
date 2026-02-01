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