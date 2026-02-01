use crate::architecture::{Register, VM};
use crate::helper_funcs::sign_extend;

pub fn store_indirect(vm: &mut VM, instr: u16) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let addr = vm.read_mem(vm.reg(Register::PC.into()).wrapping_add(pc_offset));
    vm.write_mem(addr, vm.reg(sr.into()));
}
