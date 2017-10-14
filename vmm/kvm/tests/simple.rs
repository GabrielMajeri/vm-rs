extern crate kvm;

#[test]
fn simple() {
    let device = kvm::Device::new().unwrap();

    let vm = device.create_vm().unwrap();

    let vcpu = vm.create_vcpu().unwrap();
}
