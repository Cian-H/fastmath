//lib.rs
#![allow(unused_imports)]

pub mod lookup;
mod fastmath;

pub use fastmath::*;

#[cfg(test)]
mod tests;