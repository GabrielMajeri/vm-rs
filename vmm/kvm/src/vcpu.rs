use RawFd;
use errors::Result;

use nix::unistd::close;

/// Represents a virtual CPU of the virtual machine.
///
/// Each vCPU owns and runs on an associated thread.
#[derive(Debug)]
pub struct VirtualCPU {
    fd: RawFd,
}

impl VirtualCPU {
    pub(crate) fn new(fd: RawFd) -> Result<Self> {
        let vcpu = VirtualCPU { fd };

        Ok(vcpu)
    }
}

impl Drop for VirtualCPU {
    fn drop(&mut self) {
        close(self.fd).unwrap();
    }
}
