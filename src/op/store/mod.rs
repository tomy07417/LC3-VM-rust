mod store_st;
mod store_sti;
mod store_str;

pub use store_st::store;
pub use store_sti::store_indirect;
pub use store_str::store_register;

#[cfg(test)]
mod tests {
    use super::{store, store_indirect, store_register};
    use crate::architecture::{Register, START_PC, VM};

    fn pc_relative_instr(opcode: u16, sr: u16, offset: u16) -> u16 {
        (opcode << 12) | ((sr & 0x7) << 9) | (offset & 0x1FF)
    }

    fn base_offset_instr(opcode: u16, sr: u16, base: u16, offset: u16) -> u16 {
        (opcode << 12) | ((sr & 0x7) << 9) | ((base & 0x7) << 6) | (offset & 0x3F)
    }

    #[test]
    fn st_stores_to_pc_relative_address() {
        let mut vm = VM::new();
        let pc = START_PC.wrapping_add(1);
        let offset = 0x0007;
        let target = pc.wrapping_add(offset);

        vm.set_reg(Register::PC.into(), pc);
        vm.set_reg(Register::R1.into(), 0xCAFE);

        let instr = pc_relative_instr(0x3, Register::R1 as u16, offset);
        store(&mut vm, instr);

        assert_eq!(vm.read_mem(target), 0xCAFE);
    }

    #[test]
    fn sti_stores_through_pointer_from_pc_relative_address() {
        let mut vm = VM::new();
        let pc = START_PC.wrapping_add(2);
        let offset = 0x0002;
        let pointer = 0x4200u16;

        vm.set_reg(Register::PC.into(), pc);
        vm.set_reg(Register::R2.into(), 0x0F0F);
        vm.write_mem(pc.wrapping_add(offset), pointer);

        let instr = pc_relative_instr(0xB, Register::R2 as u16, offset);
        store_indirect(&mut vm, instr);

        assert_eq!(vm.read_mem(pointer), 0x0F0F);
    }

    #[test]
    fn str_stores_to_base_plus_signed_offset() {
        let mut vm = VM::new();
        let base = 0x4100u16;
        let offset = 0x0004u16;
        let target = base.wrapping_add(offset);

        vm.set_reg(Register::R4.into(), base);
        vm.set_reg(Register::R3.into(), 0x2222);

        let instr = base_offset_instr(0x7, Register::R3 as u16, Register::R4 as u16, offset);
        store_register(&mut vm, instr);

        assert_eq!(vm.read_mem(target), 0x2222);
    }
}
