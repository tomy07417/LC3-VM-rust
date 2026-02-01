use crate::architecture::{Register, VM};
use crate::op::{decode_opcode, trap, OpCode};

/// Execute a single fetch/decode/dispatch cycle.
pub fn step(vm: &mut VM) {
    let instr: u16 = vm.read_mem(vm.reg(Register::PC.into()));

    vm.set_reg(
        Register::PC.into(),
        vm.reg(Register::PC.into()).wrapping_add(1),
    );

    let op: u16 = decode_opcode(instr);

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
