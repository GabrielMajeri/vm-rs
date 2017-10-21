extern crate kvm;

#[test]
fn add() {
    let device = kvm::Device::new().unwrap();
    let vm = device.create_vm().unwrap();
    let vcpu = vm.create_vcpu().unwrap();

    const MEMORY_SIZE: usize = 4096;
    let memory = Box::new([0u8; MEMORY_SIZE]);

    vm.set_memory_region(Some(memory.as_ptr() as usize), MEMORY_SIZE, 0, 0, 0)
        .expect("Failed to allocate memory");
}
