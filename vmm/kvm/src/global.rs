//! The KVM global object, used to create virtual machines,
//! query supported capabilities, and set process-wide settings.

use std::fs::File;
use errors;
use errors::{Result, ErrorKind};
use accel;
use kvm;
use kvm::Capability;
use vm::VirtualMachine;

#[derive(Debug)]
pub struct Global {
    file: File,
}

impl Global {
    /// Creates a handle to the KVM kernel module.
    pub fn new() -> Result<Self> {
        let file = File::open("/dev/kvm")?;

        let accel = Global { file };

        accel.check_version()?;
        accel.check_required_capabilities()?;

        Ok(accel)
    }

    /// Retrieves the raw file descriptor for this device.
    #[inline]
    fn fd(&self) -> kvm::RawFd {
        use std::os::unix::io::AsRawFd;
        self.file.as_raw_fd()
    }

    /// Checks the KVM module's API version.
    fn check_version(&self) -> Result<()> {
        let current = unsafe { kvm::get_api_version(self.fd(), 0)? };
        let expected = kvm::API_VERSION;

        // Version has to match exactly.
        if current != expected {
            bail!(ErrorKind::UnsupportedVersion(current, expected));
        }

        Ok(())
    }

    /// Checks to ensure required capabilities are present.
    fn check_required_capabilities(&self) -> Result<()> {
        const REQUIRED: &[Capability] = &[Capability::CheckExtensionVM, Capability::EmulateCpuid];

        for &cap in REQUIRED {
            self.require_capability(cap)?;
        }

        Ok(())
    }

    /// Checks if a given capability is supported.
    ///
    /// The return value is a positive number if supported,
    /// and the meaning of this value depends on the specific capability.
    pub fn check_capability(&self, cap: Capability) -> Result<u32> {
        let value = unsafe { kvm::check_extension(self.fd(), cap as i32)? };
        Ok(value)
    }

    /// Ensures a given capability is supported, otherwise returns an error.
    pub fn require_capability(&self, cap: Capability) -> Result<()> {
        if self.check_capability(cap)? == 0 {
            let cap_name = format!("{:?}", cap);
            bail!(ErrorKind::UnsupportedCapability(cap_name))
        } else {
            Ok(())
        }
    }

    /// The size of the vCPU run state structure, in bytes.
    pub fn vcpu_mmap_size(&self) -> Result<usize> {
        let size = unsafe { kvm::get_vcpu_mmap_size(self.fd())? };

        Ok(size as usize)
    }
}

impl accel::Accelerator for Global {
    fn create_vm<'a>(&'a self) -> accel::Result<Box<accel::VirtualMachine + 'a>> {
        let vm = VirtualMachine::new(self);

        Ok(Box::new(vm))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        Global::new().expect("Failed to connect to KVM kernel module");
    }
}
