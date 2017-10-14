use {Device, VirtualMachine, VirtualCPU};

pub fn get_device() -> &'static Device {
    lazy_static! {
        static ref DEVICE: Device = Device::new().unwrap();
    }

    &DEVICE
}

pub fn get_vm() -> &'static VirtualMachine<'static> {
    lazy_static! {
        static ref VM: VirtualMachine<'static> = get_device().create_vm().unwrap();
    }

    &VM
}

pub fn get_vcpu() -> &'static VirtualCPU<'static> {
    lazy_static! {
        static ref VCPU: VirtualCPU<'static> = get_vm().create_vcpu().unwrap();
    }

    &VCPU
}
