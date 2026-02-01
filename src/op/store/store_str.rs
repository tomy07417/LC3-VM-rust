use crate::architecture::VM;
use crate::helper_funcs::sign_extend;

pub fn store_register(vm: &mut VM, instr: u16) {
    let sr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);

    let addr = vm.reg(base_r.into()).wrapping_add(offset);
    vm.write_mem(addr, vm.reg(sr.into()));
}
