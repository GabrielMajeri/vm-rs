//! Generic interface for hardware-accelerated virtualization.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

#[macro_use]
extern crate error_chain;

extern crate vm_x86 as x86;

pub mod errors;
use errors::Result;

pub mod arch;

/// An accelerator takes advantage of hardware features to enable
/// fast virtualization.
pub trait Accelerator {
    /// Create a virtual machine.
    fn create_vm<'a>(&'a self) -> Result<Box<VirtualMachine<'a> + 'a>>;
}

/// A virtual machine is a group of resources such as virtual CPUs,
/// memory and hardware devices.
pub trait VirtualMachine<'a> {
    /// Recommended maximum number for virtual CPUs.
    fn max_recommended_vcpus(&self) -> Result<usize>;

    /// Maximum possible number of vCPUs.
    ///
    /// Trying to allocate more than this many vCPUs will certainly
    /// result in an error.
    fn max_vcpus(&self) -> Result<usize>;

    /// Maximum value for a virtual CPU's ID.
    fn max_vcpu_ids(&self) -> Result<usize>;

    /// Allocates a block of host memory to the VM.
    fn allocate_memory(&self, memory: MemoryRegion) -> Result<()>;

    /// Create a new virtual CPU.
    ///
    /// The `id` is a unique number identifying this CPU.
    /// On x86, this will become the APIC ID of the vCPU.
    fn create_vcpu<'b>(
        &'b self,
        id: usize,
        callbacks: &'b CpuCallbacks,
    ) -> Result<Box<VirtualCPU<'b> + 'b>>;
}

/// A virtual CPU represents a single hardware-thread in the guest VM.
///
/// For maximum performance and compatibility, each virtual CPU should
/// be assigned a thread and only run on that thread.
pub trait VirtualCPU<'a> {
    /// Synchronises the virtual CPU's state between the kernel driver
    /// and the user mode structure.
    fn sync(&self, state: &mut arch::CpuState, set: bool) -> Result<()>;

    /// Runs the virtual CPU on the current thread.
    // TODO: instead of an exit state structure,
    // we should move everything to callbacks (eventually).
    fn run(&self) -> Result<ExitState>;
}

/// A block of host memory, to be used by the guest.
///
/// Most accelerators require the memory region to be page-aligned.
///
/// Memory should be aligned on page / huge page
/// boundaries for best performance.
#[derive(Debug, Copy, Clone)]
pub struct MemoryRegion<'a> {
    /// The memory slot to operate on.
    pub slot: u8,
    /// Host virtual memory block.
    pub host: &'a [u8],
    /// Guest physical address.
    pub guest: usize,
}

/// Trait containing callbacks which control the vCPU's execution.
pub trait CpuCallbacks {
    /// Function called to emulate a port-I/O instruction.
    ///
    /// The parameters are:
    /// - `port` is the 16-bit I/O port.
    /// - `output` describes whether the data must be sent to the guest (true)
    ///   or is received (false).
    /// - `buffer` is a packed array of elements to be read or transferred.
    /// - `element_size` is the size in bytes of each element.
    fn port_io(
        &self,
        port: u16,
        output: bool,
        buffer: &mut [u8],
        element_size: usize,
    ) -> Result<()>;
}

/// Structure providing additional data on the vCPU's exit.
#[derive(Debug, Copy, Clone)]
pub enum ExitState {
    /// The virtual machine gracefully shut down.
    Shutdown,
    /// An unknown / unhandled error occured.
    Unknown(arch::ExitReason),
}
