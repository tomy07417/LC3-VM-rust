use crate::architecture::{CondFlag, Register, VM};

/// Update the condition flags based on a 16-bit register value.
///
/// Sets `COND` to one of `FlZro`, `FlNeg`, or `FlPos` using the LC-3 rules:
/// zero when the value is 0, negative when bit 15 is 1, otherwise positive.
///
/// # Examples
/// ```no_run
/// use lc3_vm_rust::architecture::{CondFlag, Register, VM};
/// use lc3_vm_rust::helper_funcs::update_flags;
///
/// let mut vm = VM::new();
///
/// update_flags(0, &mut vm);
/// assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Zro.into());
/// ```
pub fn update_flags(r: u16, vm: &mut VM) {
    if r == 0 {
        vm.set_reg(Register::Cond.into(), CondFlag::Zro.into());
    } else if r >> 15 == 1 {
        // Is negative
        vm.set_reg(Register::Cond.into(), CondFlag::Neg.into());
    } else {
        // Is positive
        vm.set_reg(Register::Cond.into(), CondFlag::Pos.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_flags_sets_zero_for_value_zero() {
        // Zero is a boundary case that must set the Z flag.
        let mut vm = VM::new();

        update_flags(0, &mut vm);

        assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Zro.into());
    }

    #[test]
    fn update_flags_sets_negative_for_msb_set() {
        // 0x8000 is the smallest negative 16-bit value (sign bit only).
        let mut vm = VM::new();

        update_flags(0x8000, &mut vm);

        assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Neg.into());
    }

    #[test]
    fn update_flags_sets_positive_for_nonzero_msb_clear() {
        // 0x0001 is the smallest positive 16-bit value.
        let mut vm = VM::new();

        update_flags(0x0001, &mut vm);

        assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Pos.into());
    }
}
