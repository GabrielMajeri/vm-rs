use errors::Result;
use {Device, VirtualCPU, ioctl_none, RawFd, KVM_IO};

use nix::unistd::close;

/// Represents a virtual machine, with some memory and virtual CPUs.
///
/// VM functions must not be accessed from another process / address space.
#[derive(Debug)]
pub struct VirtualMachine<'device> {
    device: &'device Device,
    fd: RawFd,
}

impl<'a> VirtualMachine<'a> {
    pub(crate) fn new(device: &'a Device, fd: RawFd) -> Result<Self> {
        let vm = VirtualMachine { device, fd };

        Ok(vm)
    }

    /// Adds a virtual CPU to the virtual machine.
    pub fn create_vcpu(&self) -> Result<VirtualCPU> {
        let code = io!(KVM_IO, 0x41);

        // TODO: multiple vcpus.
        let fd = ioctl_none(self.fd, code, 0)?;

        VirtualCPU::new(self, fd)
    }

    pub(crate) fn device(&self) -> &Device {
        self.device
    }
}

impl<'a> Drop for VirtualMachine<'a> {
    fn drop(&mut self) {
        close(self.fd).unwrap();
    }
}
