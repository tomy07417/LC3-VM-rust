mod load_op;
mod load_rg;
mod load_lea;

pub use load_op::load;
pub use load_rg::load_register;
pub use load_lea::load_effective_address;