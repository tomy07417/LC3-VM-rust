use crate::architecture::{VM, Register};
use crate::helper_funcs::sign_extend;

pub fn store(vm: &mut VM, instr: u16) {
    let reg = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    vm.write_mem(vm.reg(Register::PC.into()) + pc_offset, vm.reg(reg.into()));
}