//! Supports creating virtual machines using KVM (Kernel-based Virtual Machines).

#![cfg(target_os = "linux")]

#![warn(missing_docs)]
#![cfg_attr(feature="clippy", warn(clippy))]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nix;

// Stuck with this manually using `libc` until
// https://github.com/nix-rust/nix/issues/781
// gets resolved.
extern crate libc;

fn ioctl_none(fd: i32, code: u64, param: u32) -> ::errors::Result<i32> {
    let result = unsafe { libc::ioctl(fd, code, param) };

    if result < 0 {
        Err(::errors::Error::from(
            ::std::io::Error::from_raw_os_error(fd),
        ))
    } else {
        Ok(result)
    }
}

/// The type of all KVM `ioctl`s.
const KVM_IO: u8 = 0xAE;

use std::os::unix::io::RawFd;

pub mod errors;

mod ext;

mod device;
pub use device::Device;

mod vm;
pub use vm::VirtualMachine;

mod vcpu;
pub use vcpu::VirtualCPU;
