use crate::architecture::VM;
use crate::helper_funcs::update_flags;

pub fn not(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0x7;
    let sr = (instr >> 6) & 0x7;

    vm.set_reg(dr.into(), !vm.reg(sr.into()));
    update_flags(dr, vm);
}