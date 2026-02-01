use crate::architecture::{Register, VM};
use crate::op::{alu::*, decode_opcode, jump::*, load::*, store::*, trap::trap, OpCode};

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
            add(vm, instr);
        }
        x if OpCode::And == x => {
            and(vm, instr);
        }
        x if OpCode::Not == x => {
            not(vm, instr);
        }
        x if OpCode::Br == x => {
            branch(vm, instr);
        }
        x if OpCode::Jmp == x => {
            jump(vm, instr);
        }
        x if OpCode::Jsr == x => {
            jump_register(vm, instr);
        }
        x if OpCode::Ld == x => {
            load(vm, instr);
        }
        x if OpCode::Ldi == x => {
            load_indirect(vm, instr);
        }
        x if OpCode::Ldr == x => {
            load_register(vm, instr);
        }
        x if OpCode::Lea == x => {
            load_effective_address(vm, instr);
        }
        x if OpCode::St == x => {
            store(vm, instr);
        }
        x if OpCode::Sti == x => {
            store_indirect(vm, instr);
        }
        x if OpCode::Str == x => {
            store_register(vm, instr);
        }
        x if OpCode::Trap == x => {
            trap(vm, instr);
        }
        x if OpCode::Res == x => {
            eprintln!("OPCode RES - Reserved opcode encountered. Halting VM.");
        }
        x if OpCode::Rti == x => {
            eprintln!("OPCode RTI - Unimplemented opcode encountered. Halting VM.");
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::step;
    use crate::architecture::{Register, START_PC, VM};

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
