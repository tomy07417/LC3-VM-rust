mod architecture;
mod helper_funcs;
mod op;
mod step;
mod run;

use crate::architecture::{Register, CondFlag, VM, START_PC};
use crate::run::run;

fn main() {
    let mut vm = VM::new();

    vm.set_reg(Register::Cond.into(), CondFlag::Pos.into());
    vm.set_reg(Register::PC.into(), START_PC);

    run(&mut vm);
}
