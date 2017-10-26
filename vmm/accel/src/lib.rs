//! Generic interface for hardware-accelerated virtualization.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

#[macro_use]
extern crate error_chain;

pub mod errors;

/// An accelerator takes advantage of hardware features to enable
/// fast virtualization.
pub trait Accelerator {
    /// Create a virtual machine.
    fn create_vm<'a>(&'a self) -> errors::Result<Box<VirtualMachine<'a> + 'a>>;
}

/// A virtual machine is a group of resources such as virtual CPUs,
/// memory and hardware devices.
pub trait VirtualMachine<'a> {
    /// Create a new virtual CPU.
    ///
    /// The slot is a unique number identifing this CPU.
    fn create_vcpu<'b>(&'b self, slot: usize) -> errors::Result<Box<VirtualCPU<'b> + 'b>>;
}

/// A virtual CPU represents a single hardware-thread in the guest VM.
pub trait VirtualCPU<'a> {
}
