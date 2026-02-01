use crate::architecture::VM;
use crate::helper_funcs::{sign_extend, update_flags};

pub fn and(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1{
        let imm5 = sign_extend(instr & 0x1F, 5);
        vm.set_reg(dr.into(), vm.reg(sr1.into()) & imm5);
    }
    else
    {
        let sr2 = instr & 0x7;
        vm.set_reg(dr.into(), vm.reg(sr1.into()) & vm.reg(sr2.into()));
    }
    update_flags(dr, vm);
}