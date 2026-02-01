/// Sign-extend the low `bit_count` bits of `x` to a full 16-bit value.
///
/// The input is treated as a signed integer with `bit_count` width, where the
/// highest bit is the sign bit. If the sign bit is 1, the result fills the
/// remaining upper bits with 1s; otherwise it leaves them as 0s.
///
/// # Examples
/// ```
/// use lc3_vm_rust::helper_funcs::sign_extend::sign_extend;
///
/// assert_eq!(sign_extend(0b0_1111, 5), 0b0_1111);
/// assert_eq!(sign_extend(0b1_0000, 5), 0xFFF0);
/// ```
pub fn sign_extend(x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        // Negative number
        x | (0xFFFF << bit_count) // 11111... (16 bits) shifted left by bit_count
                                  // Anded to x to fill in the leading bits with 1s
    } else {
        // Positive number
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_extend_one_bit_zero() {
        // 1-bit value with sign bit 0 stays zero when extended.
        let value = 0b0u16;
        let extended = sign_extend(value, 1);

        assert_eq!(extended, 0b0u16);
    }

    #[test]
    fn sign_extend_one_bit_one() {
        // 1-bit value with sign bit 1 becomes all 1s (i.e., -1 in 16-bit).
        let value = 0b1u16;
        let extended = sign_extend(value, 1);
        assert_eq!(extended, 0xFFFFu16);
    }

    #[test]
    fn sign_extend_positive_max_for_width() {
        // Largest positive 5-bit value keeps the same bits after extension.
        let value = 0b0_1111u16;
        let extended = sign_extend(value, 5);

        assert_eq!(extended, 0b0_1111u16);
    }

    #[test]
    fn sign_extend_negative_min_for_width() {
        // Smallest negative 5-bit value (10000) extends with leading 1s.
        let value = 0b1_0000u16;
        let extended = sign_extend(value, 5);
        assert_eq!(extended, 0xFFF0u16);
    }
}
