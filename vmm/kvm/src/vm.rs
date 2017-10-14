use errors::Result;
use VirtualCPU;
use {ioctl_none, RawFd, KVM_IO};

use nix::unistd::close;

/// Represents a virtual machine, with some memory and virtual CPUs.
///
/// VM functions must not be accessed from another process / address space.
#[derive(Debug)]
pub struct VirtualMachine {
    fd: RawFd,
}

impl VirtualMachine {
    pub(crate) fn new(fd: RawFd) -> Result<Self> {
        let vm = VirtualMachine { fd };

        Ok(vm)
    }

    /// Adds a virtual CPU to the virtual machine.
    pub fn create_vcpu(&self) -> Result<VirtualCPU> {
        let code = io!(KVM_IO, 0x41);

        // TODO: multiple vcpus.
        let fd = ioctl_none(self.fd, code, 0)?;

        VirtualCPU::new(fd)
    }
}

impl Drop for VirtualMachine {
    fn drop(&mut self) {
        close(self.fd).unwrap();
    }
}
