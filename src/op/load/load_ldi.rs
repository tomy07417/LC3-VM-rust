use crate::architecture::{Register, VM};
use crate::helper_funcs::{sign_extend, update_flags};

pub fn load_indirect(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let addr = vm.read_mem(vm.reg(Register::PC.into()).wrapping_add(pc_offset));
    vm.set_reg(dr.into(), vm.read_mem(addr));
    update_flags(dr, vm);
}
