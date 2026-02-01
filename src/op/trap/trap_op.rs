use std::io::Write;

use crate::architecture::{Register, VM};
use crate::op::trap::TrapCode;

/// Execute a TRAP instruction, dispatching by trap vector.
pub fn trap(vm: &mut VM, instr: u16) {
    // Save the current PC in R7
    vm.set_reg(Register::R7.into(), vm.reg(Register::PC.into()));

    // extract the 8-bit trap vector
    match instr & 0x00FF {
        x if TrapCode::Getc == x => {}
        x if TrapCode::Out == x => {}
        x if TrapCode::Puts == x => {}
        x if TrapCode::In == x => {}
        x if TrapCode::Putsp == x => {}
        x if TrapCode::Halt == x => {
            println!("HALT");
            std::io::stdout().flush().unwrap();
            vm.shut_down();
        }
        _ => {
            panic!("Invalid trap code in instruction: {:04X}", instr);
        }
    }
}
