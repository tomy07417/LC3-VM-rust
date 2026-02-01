use crate::architecture::{VM, Register};
use crate::helper_funcs::sign_extend;

pub fn jump_register(vm: &mut VM, instr: u16) {
    let long_flag = (instr >> 11) & 1;
    vm.set_reg(Register::R7.into(), vm.reg(Register::PC.into()));

    if long_flag == 1 {
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        vm.set_reg(Register::PC.into(), vm.reg(Register::PC.into()).wrapping_add(long_pc_offset));  /* JSR */
    }
    else {
        let r1 = (instr >> 6) & 0x7;
        vm.set_reg(Register::PC.into(), vm.reg(r1.into())); /* JSRR */
    }
}