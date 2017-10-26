//! The KVM global object, used to create virtual machines,
//! query supported capabilities, and set process-wide settings.

use std::fs::File;
use accel;
use accel::errors::Result;
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
            bail!(
                "unsupported KVM version: expected {}, got {}",
                expected,
                current
            );
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
            bail!("KVM capability not supported: {}", cap_name)
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
    fn create_vm<'a>(&'a self) -> Result<Box<accel::VirtualMachine + 'a>> {
        // This is only relevant for non-x86.
        let machine_type = 0;

        let fd = unsafe { kvm::create_vm(self.fd(), machine_type)? };

        use std::os::unix::io::FromRawFd;
        let file = unsafe { File::from_raw_fd(fd as i32) };

        let vm = VirtualMachine::new(self, file)?;

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

    #[test]
    fn create_vm() {
        use accel::Accelerator;

        let g = Global::new().unwrap();

        g.create_vm().expect("Failed to create virtual machine");
    }
}
