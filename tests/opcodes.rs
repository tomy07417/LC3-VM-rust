use lc3_vm_rust::architecture::{CondFlag, Register, START_PC, VM};
use lc3_vm_rust::step::step;

fn run_single(instr: u16, setup: impl FnOnce(&mut VM)) -> VM {
    let mut vm = VM::new();
    setup(&mut vm);
    vm.write_mem(START_PC, instr);
    step(&mut vm);
    vm
}

fn instr_add_imm(dr: u16, sr1: u16, imm5: u16) -> u16 {
    (0x1 << 12) | (dr << 9) | (sr1 << 6) | (1 << 5) | (imm5 & 0x1F)
}

fn instr_add_reg(dr: u16, sr1: u16, sr2: u16) -> u16 {
    (0x1 << 12) | (dr << 9) | (sr1 << 6) | (sr2 & 0x7)
}

fn instr_and_imm(dr: u16, sr1: u16, imm5: u16) -> u16 {
    (0x5 << 12) | (dr << 9) | (sr1 << 6) | (1 << 5) | (imm5 & 0x1F)
}

fn instr_and_reg(dr: u16, sr1: u16, sr2: u16) -> u16 {
    (0x5 << 12) | (dr << 9) | (sr1 << 6) | (sr2 & 0x7)
}

fn instr_not(dr: u16, sr: u16) -> u16 {
    (0x9 << 12) | (dr << 9) | (sr << 6) | 0x3F
}

fn instr_br(cond: u16, offset9: u16) -> u16 {
    (0x0 << 12) | ((cond & 0x7) << 9) | (offset9 & 0x1FF)
}

fn instr_jmp(base_r: u16) -> u16 {
    (0xC << 12) | ((base_r & 0x7) << 6)
}

fn instr_jsr(offset11: u16) -> u16 {
    (0x4 << 12) | (1 << 11) | (offset11 & 0x7FF)
}

fn instr_jsrr(base_r: u16) -> u16 {
    (0x4 << 12) | ((base_r & 0x7) << 6)
}

fn instr_lea(dr: u16, offset9: u16) -> u16 {
    (0xE << 12) | (dr << 9) | (offset9 & 0x1FF)
}

#[test]
fn halt_instruction_stops_vm() {
    let mut vm = VM::new();
    vm.write_mem(START_PC, 0xF025);

    step(&mut vm);

    assert!(!vm.is_running());
}

#[test]
fn add_immediate_sets_register_and_pos_flag() {
    let instr = instr_add_imm(0, 1, 1);
    let vm = run_single(instr, |vm| {
        vm.set_reg(1, 1);
    });

    assert_eq!(vm.reg(0), 2);
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Pos.into());
}

#[test]
fn add_register_sets_register_and_zero_flag() {
    let instr = instr_add_reg(2, 1, 3);
    let vm = run_single(instr, |vm| {
        vm.set_reg(1, 1);
        vm.set_reg(3, 0xFFFF);
    });

    assert_eq!(vm.reg(2), 0);
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Zro.into());
}

#[test]
fn and_immediate_sets_register_and_zero_flag() {
    let instr = instr_and_imm(0, 1, 0);
    let vm = run_single(instr, |vm| {
        vm.set_reg(1, 0x00FF);
    });

    assert_eq!(vm.reg(0), 0);
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Zro.into());
}

#[test]
fn and_register_sets_register_and_pos_flag() {
    let instr = instr_and_reg(4, 2, 3);
    let vm = run_single(instr, |vm| {
        vm.set_reg(2, 0x00F0);
        vm.set_reg(3, 0x00FF);
    });

    assert_eq!(vm.reg(4), 0x00F0);
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Pos.into());
}

#[test]
fn not_sets_register_and_neg_flag() {
    let instr = instr_not(2, 1);
    let vm = run_single(instr, |vm| {
        vm.set_reg(1, 0);
    });

    assert_eq!(vm.reg(2), 0xFFFF);
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Neg.into());
}

#[test]
fn lea_writes_address_and_updates_flags() {
    let instr = instr_lea(5, 2);
    let vm = run_single(instr, |_| {});

    assert_eq!(vm.reg(5), START_PC.wrapping_add(3));
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Pos.into());
}

#[test]
fn branch_on_negative_takes_offset() {
    let instr = instr_br(CondFlag::Neg.into(), 1);
    let vm = run_single(instr, |vm| {
        vm.set_reg(Register::Cond.into(), CondFlag::Neg.into());
    });

    assert_eq!(vm.reg(Register::PC.into()), START_PC.wrapping_add(2));
}

#[test]
fn branch_on_zero_takes_offset() {
    let instr = instr_br(CondFlag::Zro.into(), 1);
    let vm = run_single(instr, |vm| {
        vm.set_reg(Register::Cond.into(), CondFlag::Zro.into());
    });

    assert_eq!(vm.reg(Register::PC.into()), START_PC.wrapping_add(2));
}

#[test]
fn branch_on_positive_takes_offset() {
    let instr = instr_br(CondFlag::Pos.into(), 1);
    let vm = run_single(instr, |vm| {
        vm.set_reg(Register::Cond.into(), CondFlag::Pos.into());
    });

    assert_eq!(vm.reg(Register::PC.into()), START_PC.wrapping_add(2));
}

#[test]
fn jmp_sets_pc_from_base_register() {
    let instr = instr_jmp(3);
    let vm = run_single(instr, |vm| {
        vm.set_reg(3, 0x4444);
        vm.set_reg(Register::Cond.into(), CondFlag::Zro.into());
    });

    assert_eq!(vm.reg(Register::PC.into()), 0x4444);
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Zro.into());
}

#[test]
fn jsr_sets_r7_and_updates_pc() {
    let instr = instr_jsr(2);
    let vm = run_single(instr, |_| {});

    assert_eq!(vm.reg(Register::R7.into()), START_PC.wrapping_add(1));
    assert_eq!(vm.reg(Register::PC.into()), START_PC.wrapping_add(3));
}

#[test]
fn jsrr_sets_r7_and_updates_pc() {
    let instr = instr_jsrr(4);
    let vm = run_single(instr, |vm| {
        vm.set_reg(4, 0x5000);
    });

    assert_eq!(vm.reg(Register::R7.into()), START_PC.wrapping_add(1));
    assert_eq!(vm.reg(Register::PC.into()), 0x5000);
}

#[test]
fn rti_opcode_does_not_modify_registers() {
    let instr = 0x8 << 12;
    let vm = run_single(instr, |vm| {
        vm.set_reg(0, 0x1111);
        vm.set_reg(Register::Cond.into(), CondFlag::Pos.into());
    });

    assert_eq!(vm.reg(0), 0x1111);
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Pos.into());
    assert_eq!(vm.reg(Register::PC.into()), START_PC.wrapping_add(1));
}

#[test]
fn res_opcode_does_not_modify_registers() {
    let instr = 0xD << 12;
    let vm = run_single(instr, |vm| {
        vm.set_reg(1, 0x2222);
        vm.set_reg(Register::Cond.into(), CondFlag::Zro.into());
    });

    assert_eq!(vm.reg(1), 0x2222);
    assert_eq!(vm.reg(Register::Cond.into()), CondFlag::Zro.into());
    assert_eq!(vm.reg(Register::PC.into()), START_PC.wrapping_add(1));
}
