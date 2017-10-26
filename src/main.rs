//! Front-end for `vm-rs`.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate accel;
extern crate kvm;

fn main() {
    let acc = kvm::create().expect("Failed to create accelerator");

    let vm = acc.create_vm().expect("Failed to create VM");

    println!(
        "Max recommended vCPUs: {}",
        vm.max_recommended_vcpus().unwrap()
    );
    println!("Max vCPUs: {}", vm.max_vcpus().unwrap());
    println!("Max vCPU IDs: {}", vm.max_vcpu_ids().unwrap());

    let _vcpu = vm.create_vcpu(0).expect("Failed to create vCPU");
}
