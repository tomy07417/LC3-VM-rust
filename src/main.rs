mod architecture;
mod helper_funcs;
mod op;
mod run;
mod step;

use crate::architecture::{CondFlag, Register, START_PC, VM};
use crate::run::run;

fn main() {
    let mut vm = VM::new();

    vm.set_reg(Register::Cond.into(), CondFlag::Pos.into());
    vm.set_reg(Register::PC.into(), START_PC);

    run(&mut vm);
}
