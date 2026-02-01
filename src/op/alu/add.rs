use crate::architecture::VM;
use crate::helper_funcs::{sign_extend, update_flags};

/// Execute the ADD instruction (register or immediate).
pub fn add(vm: &mut VM, instr: u16) {
    /* destination register (DR) */
    let dr = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let sr1 = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag = (instr >> 5) & 0x1;
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        vm.set_reg(dr.into(), vm.reg(sr1.into()).wrapping_add(imm5));
    } else {
        let sr2 = instr & 0x7;
        vm.set_reg(
            dr.into(),
            vm.reg(sr1.into()).wrapping_add(vm.reg(sr2.into())),
        );
    }

    update_flags(dr, vm);
}
