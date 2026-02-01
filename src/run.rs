use crate::architecture::VM;
use crate::step::step;

/// Execute VM steps until the machine halts.
pub fn run(vm: &mut VM) {
    while vm.is_running() {
        step(vm);
    }
}
