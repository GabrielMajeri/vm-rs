//! Support for Intel's Hardware Accelerated Execution Manager.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

#[macro_use]
extern crate error_chain;

extern crate accel;

extern crate hax_sys as hax;

extern crate vm_x86 as x86;

/// Creates an object which implements the `Accelerator` trait.
pub fn create() -> accel::errors::Result<Box<accel::Accelerator>> {
    unimplemented!("todo")
}
