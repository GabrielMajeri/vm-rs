use errors::Result;
use {Device, VirtualCPU, ioctl_none, RawFd, KVM_IO};
use std::cmp;
use ext::Extension;
use object::Object;

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

    pub(crate) fn device(&self) -> &Device {
        self.device
    }

    /// Adds a virtual CPU.
    pub fn create_vcpu(&self) -> Result<VirtualCPU> {
        let code = io!(KVM_IO, 0x41);

        // TODO: multiple vcpus.
        let fd = ioctl_none(self.fd, code, 0)?;

        VirtualCPU::new(self, fd)
    }

    /// Returns the maximum number of virtual CPUs supported by this virtual machine.
    pub fn max_vcpus(&self) -> usize {
        let max = self.extension_supported(Extension::MaxVCPUs);

        cmp::min(1, max as usize)
    }
    }
}

impl<'a> Drop for VirtualMachine<'a> {
    fn drop(&mut self) {
        close(self.fd).unwrap();
    }
}

impl<'a> Object for VirtualMachine<'a> {
    fn fd(&self) -> RawFd {
        self.fd
    }
}
