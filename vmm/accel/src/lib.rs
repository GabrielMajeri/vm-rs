//! Generic interface for hardware-accelerated virtualization.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

#[macro_use]
extern crate error_chain;

mod errors;
pub use errors::{Result, Error};

pub trait Accelerator {
}

pub trait VirtualMachine {
}

pub trait VirtualCPU {
}
