mod load_ld;
mod load_lrg;
mod load_lea;

pub use load_ld::load;
pub use load_lrg::load_register;
pub use load_lea::load_effective_address;