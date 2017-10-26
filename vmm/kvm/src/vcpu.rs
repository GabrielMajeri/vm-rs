use accel;
use accel::errors::Result;
use vm::VirtualMachine;
use std::fs::File;

pub struct VirtualCPU<'a> {
    vm: &'a VirtualMachine<'a>,
    file: File,
}

impl<'a> VirtualCPU<'a> {
    /// Initializes the virtual CPU.
    pub fn new(vm: &'a VirtualMachine, file: File) -> Result<Self> {
        let vcpu = VirtualCPU { vm, file };

        Ok(vcpu)
    }
}

impl<'a> accel::VirtualCPU<'a> for VirtualCPU<'a> {}
