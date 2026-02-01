use crate::architecture::{VM, Register};

pub fn jump(vm: &mut VM, instr: u16) {
    /* Also handles RET */
    let r1 = (instr >> 6) & 0x7;
    
    vm.set_reg(Register::PC.into(), vm.reg(r1.into()));
}