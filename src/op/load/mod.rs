mod load_ld;
mod load_ldi;
mod load_ldr;
mod load_lea;

pub use load_ld::load;
pub use load_ldi::load_indirect;
pub use load_ldr::load_register;
pub use load_lea::load_effective_address;

#[cfg(test)]
mod tests {
    use super::{load, load_effective_address, load_indirect, load_register};
    use crate::architecture::{Register, START_PC, VM};

    fn pc_relative_instr(opcode: u16, dr: u16, offset: u16) -> u16 {
        (opcode << 12) | ((dr & 0x7) << 9) | (offset & 0x1FF)
    }

    fn base_offset_instr(opcode: u16, dr: u16, base: u16, offset: u16) -> u16 {
        (opcode << 12) | ((dr & 0x7) << 9) | ((base & 0x7) << 6) | (offset & 0x3F)
    }

    #[test]
    fn ld_loads_from_pc_relative_address() {
        let mut vm = VM::new();
        let pc = START_PC.wrapping_add(1);
        let offset = 0x0005;
        let target = pc.wrapping_add(offset);

        vm.set_reg(Register::PC.into(), pc);
        vm.write_mem(target, 0xABCD);

        let instr = pc_relative_instr(0x2, Register::R1 as u16, offset);
        load(&mut vm, instr);

        assert_eq!(vm.reg(Register::R1.into()), 0xABCD);
    }

    #[test]
    fn ldi_loads_through_pointer_from_pc_relative_address() {
        let mut vm = VM::new();
        let pc = START_PC.wrapping_add(2);
        let offset = 0x0003;
        let pointer = 0x4000u16;
        let value = 0xBEEF;

        vm.set_reg(Register::PC.into(), pc);
        vm.write_mem(pc.wrapping_add(offset), pointer);
        vm.write_mem(pointer, value);

        let instr = pc_relative_instr(0xA, Register::R2 as u16, offset);
        load_indirect(&mut vm, instr);

        assert_eq!(vm.reg(Register::R2.into()), value);
    }

    #[test]
    fn ldr_loads_from_base_plus_signed_offset() {
        let mut vm = VM::new();
        let base = 0x4100u16;
        let offset = 0x3Eu16; // -2 in 6-bit signed
        let target = base.wrapping_add(0xFFFE);

        vm.set_reg(Register::R3.into(), base);
        vm.write_mem(target, 0x1234);

        let instr = base_offset_instr(0x6, Register::R1 as u16, Register::R3 as u16, offset);
        load_register(&mut vm, instr);

        assert_eq!(vm.reg(Register::R1.into()), 0x1234);
    }

    #[test]
    fn lea_loads_effective_address_from_pc_relative_offset() {
        let mut vm = VM::new();
        let pc = START_PC.wrapping_add(4);
        let offset = 0x0010;

        vm.set_reg(Register::PC.into(), pc);

        let instr = pc_relative_instr(0xE, Register::R4 as u16, offset);
        load_effective_address(&mut vm, instr);

        assert_eq!(vm.reg(Register::R4.into()), pc.wrapping_add(offset));
    }
}
