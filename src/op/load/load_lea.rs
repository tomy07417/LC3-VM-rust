use crate::architecture::{Register, VM};
use crate::helper_funcs::{sign_extend, update_flags};

/// Execute the LEA instruction (load effective address).
///
/// Writes `PC + offset9` into the destination register and updates flags.
pub fn load_effective_address(vm: &mut VM, instr: u16) {
    // Extract bits 9-11 - destination register (DR)
    let dr = (instr >> 9) & 0x7;

    let pc_offset = sign_extend(instr & 0x1FF, 9);

    vm.set_reg(
        dr.into(),
        vm.reg(Register::PC.into()).wrapping_add(pc_offset),
    );
    update_flags(dr, vm);
}
