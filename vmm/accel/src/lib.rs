//! Generic interface for hardware-accelerated virtualization.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

#[macro_use]
extern crate error_chain;

mod errors;
pub use errors::{Result, Error};

/// An accelerator takes advantage of hardware features to enable
/// fast virtualization.
pub trait Accelerator {
    /// Create a virtual machine.
    fn create_vm<'a>(&'a self) -> Result<Box<VirtualMachine<'a> + 'a>>;
}

/// A virtual machine is a group of resources such as virtual CPUs,
/// memory and hardware devices.
pub trait VirtualMachine<'a> {
}

/// A virtual CPU represents a single hardware-thread in the guest VM.
pub trait VirtualCPU {
}
