//! Contains code common to all virtual machines.

#![deny(warnings, missing_docs)]

/// Type used to store host memory addresses.
pub type HostAddress = usize;
/// Type used to store guest memory addresses.
pub type GuestAddress = usize;

mod memory_block;
pub use self::memory_block::MemoryBlock;

mod memory_region;
pub use self::memory_region::MemoryRegion;
