//! Client for Intel's Hardware Accelerated Execution Manager.
//!
//! HAX provides a kernel-mode driver which allows clients running in user-mode to access
//! hardware virtualization features.

#![cfg(target_os = "windows")]

#![deny(warnings, missing_docs)]

#![feature(const_fn)]

extern crate vm_common as common;
extern crate x86;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
#[macro_use] extern crate bitflags;

mod errors;
pub use self::errors::*;

mod io_codes;

mod handle;
use self::handle::Handle;

mod version;

mod connection;
pub use self::connection::Connection;

mod vm;
pub use self::vm::{VirtualMachine, MemoryRegionFlags};

/// Contains structures representing HAX's virtual CPU state.
pub mod vcpu;
