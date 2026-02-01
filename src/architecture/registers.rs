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
    Cond, // Condition Flags
    Count,
}

#[cfg(test)]
mod tests {
    use super::Register;

    #[test]
    fn register_indices_match_spec() {
        assert_eq!(Register::R0 as usize, 0);
        assert_eq!(Register::R1 as usize, 1);
        assert_eq!(Register::R2 as usize, 2);
        assert_eq!(Register::R3 as usize, 3);
        assert_eq!(Register::R4 as usize, 4);
        assert_eq!(Register::R5 as usize, 5);
        assert_eq!(Register::R6 as usize, 6);
        assert_eq!(Register::R7 as usize, 7);
        assert_eq!(Register::PC as usize, 8);
        assert_eq!(Register::Cond as usize, 9);
        assert_eq!(Register::Count as usize, 10);
    }
}
