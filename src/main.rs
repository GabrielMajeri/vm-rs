//! Front-end for `vm-rs`.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate accel;
extern crate kvm;

fn main() {
    let acc = kvm::create()
        .expect("Failed to create accelerator");

    let _vm = acc.create_vm()
        .expect("Failed to create VM");
}
