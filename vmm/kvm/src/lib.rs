//! Hardware-accelerated virtualization using the [Kernel-based Virtual Machine][kvm] module.
//!
//! [kvm]: [https://en.wikipedia.org/wiki/Kernel-based_Virtual_Machine]
//!
//! # Usage
//! Use the [`create`](fn.create.html) function to create an `Accelerator` object.
//!
//! # More documentation
//! - KVM API from the [Linux kernel docs][api];
//! - QEMU accelerator code (both the [generic code][kvm-all]
//!   and the [platform-specific one][kvm-i386]);
//!
//! [api]: https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt
//! [kvm-all]: https://github.com/qemu/qemu/blob/master/accel/kvm/kvm-all.c
//! [kvm-i386]: https://github.com/qemu/qemu/blob/master/target/i386/kvm.c

#![warn(missing_docs, missing_copy_implementations, missing_debug_implementations)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

#[macro_use]
extern crate error_chain;

extern crate accel;

extern crate kvm_sys as kvm;

extern crate vm_x86 as x86;

extern crate memmap;

mod global;
mod vm;
mod vcpu;

/// Creates an object which implements the `Accelerator` trait.
pub fn create() -> accel::errors::Result<Box<accel::Accelerator>> {
    let global = global::Global::new()?;
    Ok(Box::new(global))
}
