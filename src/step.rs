use crate::architecture::{Register, VM};
use crate::op::{OpCode, decode_opcode, trap::trap};

/// Execute a single fetch/decode/dispatch cycle.
pub fn step(vm: &mut VM) {
    let instr: u16 = vm.read_mem(vm.reg(Register::PC.into()));

    vm.set_reg(
        Register::PC.into(),
        vm.reg(Register::PC.into()).wrapping_add(1),
    );

    let op: u16 = decode_opcode(instr);

    match op {
        x if OpCode::Add == x => {}
        x if OpCode::And == x => {}
        x if OpCode::Not == x => {}
        x if OpCode::Br == x => {}
        x if OpCode::Jmp == x => {}
        x if OpCode::Jsr == x => {}
        x if OpCode::Ld == x => {}
        x if OpCode::Ldi == x => {}
        x if OpCode::Ldr == x => {}
        x if OpCode::Lea == x => {}
        x if OpCode::St == x => {}
        x if OpCode::Sti == x => {}
        x if OpCode::Str == x => {}
        x if OpCode::Trap == x => {
            trap(vm, instr);
        }
        x if OpCode::Res == x => {}
        x if OpCode::Rti == x => {}
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::step;
    use crate::architecture::{Register, START_PC, VM};

    #[test]
    fn halt_instruction_stops_vm() {
        let mut vm = VM::new();
        vm.write_mem(START_PC, 0xF025);

        step(&mut vm);

        assert!(!vm.is_running());
    }

    #[test]
    fn pc_increments_between_steps() {
        let mut vm = VM::new();
        vm.write_mem(START_PC, 0x1000);
        vm.write_mem(START_PC.wrapping_add(1), 0x1000);

        let pc_before = vm.reg(Register::PC.into());

        step(&mut vm);
        assert_eq!(vm.reg(Register::PC.into()), pc_before.wrapping_add(1));

        step(&mut vm);
        assert_eq!(vm.reg(Register::PC.into()), pc_before.wrapping_add(2));
    }
}
