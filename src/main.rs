//! Front-end for `vm-rs`.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate accel;
#[cfg(target_os = "linux")]
extern crate kvm;
#[cfg(target_os = "windows")]
extern crate hax;
extern crate vm_x86 as x86;

#[cfg(target_os = "linux")]
fn create_accelerator() -> Box<accel::Accelerator> {
    kvm::create().expect("Failed to create KVM accelerator")
}

#[cfg(target_os = "windows")]
fn create_accelerator() -> Box<accel::Accelerator> {
    hax::create().expect("Failed to create HAX accelerator")
}

fn main() {
    let acc = create_accelerator();

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

            // Output to port.
            mem[2] = 0xE6;
            // The actual port.
            // TODO: come up with a port for debugging.
            mem[3] = 0;
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

    struct CpuCallbacks;

    impl accel::CpuCallbacks for CpuCallbacks {
        fn port_io(
            &self,
            port: u16,
            _output: bool,
            buffer: &mut [u8],
            _element_size: usize,
        ) -> accel::errors::Result<()> {
            println!("I/O on port {:X}", port);
            println!("Data: {:?}", buffer);

            Ok(())
        }
    }

    let cbs = CpuCallbacks;

    let vcpu = vm.create_vcpu(0, &cbs).expect("Failed to create vCPU");

    let _exit_state = vcpu.run().expect("Failed to run vCPU");
}
