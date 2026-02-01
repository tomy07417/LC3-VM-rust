use crate::architecture::{Register, VM};
use crate::helper_funcs::sign_extend;

/// Execute the BR instruction using the current condition flags.
pub fn branch(vm: &mut VM, instr: u16) {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;

    if (cond_flag & vm.reg(Register::Cond.into())) != 0 {
        vm.set_reg(
            Register::PC.into(),
            vm.reg(Register::PC.into()).wrapping_add(pc_offset),
        );
    }
}
