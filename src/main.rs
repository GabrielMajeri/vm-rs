//! Front-end for `vm-rs`.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate accel;
extern crate kvm;
extern crate vm_x86 as x86;

fn main() {
    let acc = kvm::create().expect("Failed to create accelerator");

    let vm = acc.create_vm().expect("Failed to create VM");

    let memory = {
        let mut memory = Box::new([0u8; 4096]);

        // Write some instructions to that memory.
        {
            let reset_vector = 4096 - 16;
            let mem = &mut memory[reset_vector..];

            // Move imm8 to AL.
            mem[0] = 0xB0;
            mem[1] = 127;

            // Halt.
            mem[2] = 0xF4;
        }

        memory
    };

    let region = accel::MemoryRegion {
        slot: 0,
        host: memory.as_ref(),
        guest: 4 * 1024 * 1024 * 1024 - 4096,
    };

    vm.allocate_memory(region).expect(
        "Failed to allocate memory",
    );

    let max_recommended_vcpus = vm.max_recommended_vcpus().unwrap();
    println!("Max recommended vCPUs: {}", max_recommended_vcpus);
    let max_vcpus = vm.max_vcpus().unwrap();
    println!("Max vCPUs: {}", max_vcpus);
    let max_vcpu_ids = vm.max_vcpu_ids().unwrap();
    println!("Max vCPU IDs: {}", max_vcpu_ids);

    let vcpu = vm.create_vcpu(0).expect("Failed to create vCPU");

    let mut state = x86::state::State::default();

    vcpu.sync(&mut state, false).unwrap();

    let exit_state = vcpu.run().expect("Failed to run vCPU");

    vcpu.sync(&mut state, false).unwrap();

    println!("{:X}", state.r[0]);
}
