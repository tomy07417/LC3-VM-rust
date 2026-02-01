use crate::architecture::{Register, VM};
use crate::helper_funcs::sign_extend;

/// Execute the ST instruction (PC-relative store).
///
/// Stores the source register value to memory at `PC + offset9`.
pub fn store(vm: &mut VM, instr: u16) {
    let reg = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    vm.write_mem(
        vm.reg(Register::PC.into()).wrapping_add(pc_offset),
        vm.reg(reg.into()),
    );
}
