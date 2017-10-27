//! Front-end for `vm-rs`.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate accel;
extern crate kvm;
extern crate vm_x86 as x86;

fn main() {
    let acc = kvm::create().expect("Failed to create accelerator");

    let vm = acc.create_vm().expect("Failed to create VM");

    let max_recommended_vcpus = vm.max_recommended_vcpus().unwrap();
    println!("Max recommended vCPUs: {}", max_recommended_vcpus);
    let max_vcpus = vm.max_vcpus().unwrap();
    println!("Max vCPUs: {}", max_vcpus);
    let max_vcpu_ids = vm.max_vcpu_ids().unwrap();
    println!("Max vCPU IDs: {}", max_vcpu_ids);

    let vcpu = vm.create_vcpu(0).expect("Failed to create vCPU");

    let mut state = x86::state::State::default();

    vcpu.sync(&mut state, true).unwrap();

    // vcpu.run();

    vcpu.sync(&mut state, false).unwrap();
}
