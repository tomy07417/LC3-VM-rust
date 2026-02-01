use crate::architecture::VM;
use crate::helper_funcs::{sign_extend, update_flags};

pub fn load_register(vm: &mut VM, instr: u16) {
    // Extract bits 9-11 - destination register (DR)
    let dr = (instr << 9) & 0x7;
    // Extract bits 6-8 - memory direction register (MDR)
    let mdr = (instr << 6) & 0x7;

    let offset = sign_extend(instr & 0x3F, 6);

    let md = vm.reg(mdr.into());
    vm.set_reg(dr.into(), vm.read_mem(md + offset));
    update_flags(dr, vm);
}