//! The editing core: every structural change to a document goes through
//! [`core::Core`].

pub mod core;
#[cfg(test)]
mod fuzz_tests;
pub mod table_ops;
