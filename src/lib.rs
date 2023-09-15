//lib.rs
#![allow(unused_imports)]

pub mod lookup;
pub mod macros;

mod fastmath;
pub use fastmath::*;

#[cfg(test)]
pub(crate) mod tests;