use lc3_vm_rust::architecture::{Register, START_PC, VM};
use lc3_vm_rust::step::step;

fn instr_ld(dr: u16, offset9: u16) -> u16 {
    (0x2 << 12) | ((dr & 0x7) << 9) | (offset9 & 0x1FF)
}

fn instr_ldi(dr: u16, offset9: u16) -> u16 {
    (0xA << 12) | ((dr & 0x7) << 9) | (offset9 & 0x1FF)
}

fn instr_ldr(dr: u16, base: u16, offset6: u16) -> u16 {
    (0x6 << 12) | ((dr & 0x7) << 9) | ((base & 0x7) << 6) | (offset6 & 0x3F)
}

fn instr_st(sr: u16, offset9: u16) -> u16 {
    (0x3 << 12) | ((sr & 0x7) << 9) | (offset9 & 0x1FF)
}

fn instr_sti(sr: u16, offset9: u16) -> u16 {
    (0xB << 12) | ((sr & 0x7) << 9) | (offset9 & 0x1FF)
}

fn instr_str(sr: u16, base: u16, offset6: u16) -> u16 {
    (0x7 << 12) | ((sr & 0x7) << 9) | ((base & 0x7) << 6) | (offset6 & 0x3F)
}

#[test]
fn program_loads_using_each_addressing_mode() {
    let mut vm = VM::new();

    let ld_offset = 0x0010;
    let ldi_offset = 0x0011;
    let ldr_offset = 0x0002;

    vm.write_mem(START_PC, instr_ld(Register::R1 as u16, ld_offset));
    vm.write_mem(
        START_PC.wrapping_add(1),
        instr_ldi(Register::R2 as u16, ldi_offset),
    );
    vm.write_mem(
        START_PC.wrapping_add(2),
        instr_ldr(Register::R3 as u16, Register::R4 as u16, ldr_offset),
    );

    let ld_target = START_PC.wrapping_add(1).wrapping_add(ld_offset);
    vm.write_mem(ld_target, 0x1111);

    let ldi_pointer_addr = START_PC.wrapping_add(2).wrapping_add(ldi_offset);
    let ldi_target = 0x5000u16;
    vm.write_mem(ldi_pointer_addr, ldi_target);
    vm.write_mem(ldi_target, 0x2222);

    let ldr_base = 0x6000u16;
    vm.set_reg(Register::R4.into(), ldr_base);
    vm.write_mem(ldr_base.wrapping_add(ldr_offset), 0x3333);

    step(&mut vm);
    step(&mut vm);
    step(&mut vm);

    assert_eq!(vm.reg(Register::R1.into()), 0x1111);
    assert_eq!(vm.reg(Register::R2.into()), 0x2222);
    assert_eq!(vm.reg(Register::R3.into()), 0x3333);
}

#[test]
fn program_stores_using_each_addressing_mode() {
    let mut vm = VM::new();

    let st_offset = 0x0010;
    let sti_offset = 0x0011;
    let str_offset = 0x0002;

    vm.write_mem(START_PC, instr_st(Register::R1 as u16, st_offset));
    vm.write_mem(
        START_PC.wrapping_add(1),
        instr_sti(Register::R2 as u16, sti_offset),
    );
    vm.write_mem(
        START_PC.wrapping_add(2),
        instr_str(Register::R3 as u16, Register::R4 as u16, str_offset),
    );

    vm.set_reg(Register::R1.into(), 0xAAAA);
    vm.set_reg(Register::R2.into(), 0xBBBB);
    vm.set_reg(Register::R3.into(), 0xCCCC);
    vm.set_reg(Register::R4.into(), 0x7000u16);

    let st_target = START_PC.wrapping_add(1).wrapping_add(st_offset);
    let sti_pointer_addr = START_PC.wrapping_add(2).wrapping_add(sti_offset);
    let sti_target = 0x7100u16;
    vm.write_mem(sti_pointer_addr, sti_target);

    step(&mut vm);
    step(&mut vm);
    step(&mut vm);

    assert_eq!(vm.read_mem(st_target), 0xAAAA);
    assert_eq!(vm.read_mem(sti_target), 0xBBBB);
    assert_eq!(vm.read_mem(0x7000u16.wrapping_add(str_offset)), 0xCCCC);
}

#[test]
fn sign_extension_and_wrapping_work_for_base_offset_modes() {
    let mut vm = VM::new();

    vm.write_mem(
        START_PC,
        instr_ldr(Register::R1 as u16, Register::R4 as u16, 0x3F),
    );
    vm.write_mem(
        START_PC.wrapping_add(1),
        instr_str(Register::R2 as u16, Register::R5 as u16, 0x3E),
    );

    vm.set_reg(Register::R4.into(), 0x4000u16);
    vm.write_mem(0x3FFF, 0x1234);

    vm.set_reg(Register::R5.into(), 0x0001u16);
    vm.set_reg(Register::R2.into(), 0xBEEF);

    step(&mut vm);
    step(&mut vm);

    assert_eq!(vm.reg(Register::R1.into()), 0x1234);
    assert_eq!(vm.read_mem(0xFFFF), 0xBEEF);
}
