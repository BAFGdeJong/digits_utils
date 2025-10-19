pub mod to_digits;
pub mod digit_size;
mod abs_assigner;

#[cfg(not(feature = "no_std"))]
pub mod to_digits_collection;