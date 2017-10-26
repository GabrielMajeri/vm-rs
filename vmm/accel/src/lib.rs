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
    /// Recommended maximum number for virtual CPUs.
    fn max_recommended_vcpus(&self) -> errors::Result<usize>;

    /// Maximum possible number of vCPUs.
    ///
    /// Trying to allocate more than this many vCPUs will certainly
    /// result in an error.
    fn max_vcpus(&self) -> errors::Result<usize>;

    /// Maximum value for a virtual CPU's ID.
    fn max_vcpu_ids(&self) -> errors::Result<usize>;

    /// Create a new virtual CPU.
    ///
    /// The `id` is a unique number identifying this CPU.
    /// On x86, this will become the APIC ID of the vCPU.
    fn create_vcpu<'b>(&'b self, id: usize) -> errors::Result<Box<VirtualCPU<'b> + 'b>>;
}

/// A virtual CPU represents a single hardware-thread in the guest VM.
pub trait VirtualCPU<'a> {}
