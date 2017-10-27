use accel;
use accel::errors::Result;
use global::Global;
use std::fs::File;
use kvm;
use kvm::Capability;
use vcpu::VirtualCPU;

pub struct VirtualMachine<'a> {
    global: &'a Global,
    file: File,
}

impl<'a> VirtualMachine<'a> {
    /// Initializes a new virtual machine.
    pub fn new(global: &'a Global, file: File) -> Result<Self> {
        let vm = VirtualMachine { global, file };

        vm.check_required_capabilities()?;

        vm.create_interrupt_controller()?;

        Ok(vm)
    }

    /// Retrieves the raw file descriptor for this device.
    #[inline]
    fn fd(&self) -> kvm::RawFd {
        use std::os::unix::io::AsRawFd;
        self.file.as_raw_fd()
    }

    /// Checks to ensure required capabilities are present.
    fn check_required_capabilities(&self) -> Result<()> {
        const REQUIRED: &[Capability] = &[Capability::IrqChip, Capability::UserMemory];

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
        let value = unsafe { kvm::ioctl::check_extension(self.fd(), cap as i32)? };
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

    /// Creates an in-kernel interrupt controler model.
    ///
    /// # Architecture specific details
    /// ## x86
    /// Creates a virtual I/O APIC, two virtual PICs,
    /// and each virtual CPU will have a local APIC.
    ///
    /// Global System Interrupts 0-15 are routed to both the PIC and I/O APIC.
    /// GSIs 16-23 only go to to the I/O APIC.
    fn create_interrupt_controller(&self) -> Result<()> {
        unsafe { kvm::ioctl::create_irq_chip(self.fd())? };
        Ok(())
    }

    /// Reads the state of a in-kernel interrupt controller.
    fn get_interrupt_controller(&self) -> Result<()> {
        bail!("err")
    }
}

impl<'a> accel::VirtualMachine<'a> for VirtualMachine<'a> {
    fn max_recommended_vcpus(&self) -> Result<usize> {
        self.check_capability(Capability::MaxRecommendedVCpus)
            .map(|value| value as usize)
            .or_else(|_| Ok(4))
    }

    fn max_vcpus(&self) -> Result<usize> {
        self.check_capability(Capability::MaxVCpus)
            .map(|value| value as usize)
            .or_else(|_| self.max_recommended_vcpus())
    }

    fn max_vcpu_ids(&self) -> Result<usize> {
        self.check_capability(Capability::MaxVCpuId)
            .map(|value| value as usize)
            .or_else(|_| self.max_vcpus())
    }

    fn create_vcpu<'b>(&'b self, slot: usize) -> Result<Box<accel::VirtualCPU<'b> + 'b>> {
        let slot = slot as i32;

        let fd = unsafe { kvm::ioctl::create_vcpu(self.fd(), slot)? };

        use std::os::unix::io::FromRawFd;
        let file = unsafe { File::from_raw_fd(fd as i32) };

        let vcpu = VirtualCPU::new(self, file)?;

        Ok(Box::new(vcpu))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
