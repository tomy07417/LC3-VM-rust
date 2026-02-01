use lc3_vm_rust::architecture::{CondFlag, Register, START_PC, VM};
use lc3_vm_rust::run::run;

fn main() {
    let mut vm = VM::new();

    vm.set_reg(Register::Cond.into(), CondFlag::Pos.into());
    vm.set_reg(Register::PC.into(), START_PC);

    run(&mut vm);
}
