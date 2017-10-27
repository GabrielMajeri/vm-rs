//! Low-level, unsafe bindings to the Kernel-based Virtual Machine interface.

#[macro_use]
extern crate nix;

#[macro_use]
extern crate bitflags;

extern crate vm_x86 as x86;

pub mod errors;

mod constants;
pub use constants::*;

mod caps;
pub use caps::Capability;

pub mod ioctl;

pub mod structs;

/// The KVM API is based on devices, represented as files.
/// File descriptors are handles to those files.
pub use std::os::unix::io::RawFd;
