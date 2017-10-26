#![feature(decl_macro)]

#[macro_use]
extern crate nix;

#[macro_use]
extern crate bitflags;

mod macros;

mod errors;
pub use errors::*;

mod constants;
pub use constants::*;

mod caps;
pub use caps::Capability;

mod ioctl;
pub use ioctl::*;

mod structs;
pub use structs::*;

/// The KVM API is based on devices, represented as files.
/// File descriptors are handles to those files.
pub use std::os::unix::io::RawFd;
