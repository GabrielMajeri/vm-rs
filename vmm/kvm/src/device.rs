use errors::Result;
use std::fs::File;
use ext::Extension;
use object::Object;

use {ioctl_none, KVM_IO, RawFd, VirtualMachine};

use nix::unistd::close;

/// Controls global KVM state.
#[derive(Debug)]
pub struct Device {
    fd: RawFd,
}

impl Device {
    /// Opens a connection to the KVM device.
    ///
    /// Fails if KVM is not supported / enabled on the running machine,
    /// or if an unsupported version of KVM is detected.
    pub fn new() -> Result<Self> {
        let device = {
            let file = File::open("/dev/kvm")?;

            let fd = {
                use std::os::unix::io::IntoRawFd;
                file.into_raw_fd()
            };

            Device { fd }
        };

        device.check_api_version()?;

        device.check_required_extensions()?;

        Ok(device)
    }

    /// Creates a new virtual machine.
    ///
    /// The VM starts with no memory or vCPUs allocated.
    pub fn create_vm(&self) -> Result<VirtualMachine> {
        let code = io!(KVM_IO, 0x01);

        // TODO: support VM type flags.
        let vm_type = 0;

        let fd = ioctl_none(self.fd, code, vm_type)?;

        VirtualMachine::new(self, fd)
    }

    fn check_api_version(&self) -> Result<()> {
        let code = io!(KVM_IO, 0x00);

        let version = ioctl_none(self.fd, code, 0)?;

        const KVM_API_VERSION: i32 = 12;

        if version != KVM_API_VERSION {
            bail!("KVM version mismatch")
        } else {
            Ok(())
        }
    }

    fn check_required_extensions(&self) -> Result<()> {
        const REQUIRED: &[Extension] = &[Extension::UserMemory];

        let supported = REQUIRED.iter().all(
            |&ext| self.extension_supported(ext) == 1,
        );

        if !supported {
            bail!("Required KVM capabilities not supported")
        } else {
            Ok(())
        }
    }

    pub(crate) fn vcpu_mmap_size(&self) -> Result<usize> {
        let code = io!(KVM_IO, 0x04);

        let size = ioctl_none(self.fd, code, 0)?;

        Ok(size as usize)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        close(self.fd).unwrap();
    }
}

impl Object for Device {
    fn fd(&self) -> RawFd {
        self.fd
    }
}
