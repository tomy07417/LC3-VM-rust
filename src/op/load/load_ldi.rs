use crate::architecture::{Register, VM};
use crate::helper_funcs::{sign_extend, update_flags};

/// Execute the LDI instruction (PC-relative indirect load).
///
/// Reads a pointer from `PC + offset9`, then loads the word at that address
/// into the destination register and updates condition flags.
pub fn load_indirect(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let addr = vm.read_mem(vm.reg(Register::PC.into()).wrapping_add(pc_offset));
    vm.set_reg(dr.into(), vm.read_mem(addr));
    update_flags(dr, vm);
}
