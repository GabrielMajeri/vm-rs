//! Front-end for `vm-rs`.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate accel;
extern crate kvm;

fn main() {
    let acc = kvm::create()
        .expect("Failed to create accelerator");

    let vm = acc.create_vm()
        .expect("Failed to create VM");

    let _vcpu = vm.create_vcpu(0)
        .expect("Failed to create vCPU");
}
