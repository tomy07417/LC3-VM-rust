use crate::architecture::{VM, Register};
use crate::helper_funcs::{sign_extend, update_flags};

pub fn load(vm: &mut VM, instr: u16) {
    // Extract bits 9-11 - destination register (DR)
    let dr: u16 = (instr >> 9) & 0x7;

    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let pc = vm.reg(Register::PC.into());

    // Store in dr and update flags
    vm.set_reg(dr.into(), vm.read_mem(pc + pc_offset));
    update_flags(dr, vm);
}