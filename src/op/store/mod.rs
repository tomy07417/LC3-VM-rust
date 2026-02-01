mod store_st;
mod store_sti;
mod store_str;

pub use store_st::store;
pub use store_sti::store_indirect;
pub use store_str::store_register;