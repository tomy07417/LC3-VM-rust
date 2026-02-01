use crate::architecture::{Register, VM};
use crate::op::{trap, OpCode};

/// Execute a single fetch/decode/dispatch cycle.
pub fn step(vm: &mut VM) {
    let instr: u16 = vm.read_mem(vm.reg(Register::PC.into()));

    vm.set_reg(
        Register::PC.into(),
        vm.reg(Register::PC.into()).wrapping_add(1),
    );

    let op: u16 = instr >> 12;

    match op {
        x if OpCode::Add == x => {
            return;
        }
        x if OpCode::And == x => {
            return;
        }
        x if OpCode::Not == x => {
            return;
        }
        x if OpCode::Br == x => {
            return;
        }
        x if OpCode::Jmp == x => {
            return;
        }
        x if OpCode::Jsr == x => {
            return;
        }
        x if OpCode::Ld == x => {
            return;
        }
        x if OpCode::Ldi == x => {
            return;
        }
        x if OpCode::Ldr == x => {
            return;
        }
        x if OpCode::Lea == x => {
            return;
        }
        x if OpCode::St == x => {
            return;
        }
        x if OpCode::Sti == x => {
            return;
        }
        x if OpCode::Str == x => {
            return;
        }
        x if OpCode::Trap == x => {
            trap(vm, instr);
        }
        x if OpCode::Res == x => {
            return;
        }
        x if OpCode::Rti == x => {
            return;
        }
        _ => {}
    }
}
