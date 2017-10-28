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

        vm.set_identity_mapping()?;
        vm.set_tss_address()?;

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
        const REQUIRED: &[Capability] = &[
            Capability::IrqChip,
            Capability::UserMemory,
            Capability::ReadOnlyMemory,
            Capability::SetIdentityMapAddress,
            Capability::SetTssAddr,
        ];

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

    /// Returns the base address of the EPT.
    fn ept_address(&self) -> u64 {
        // Reserve up to 16-MiB of memory for the BIOS.
        let bios_base = 0xFF00_0000;

        let page_size = 4096;

        // We will use the 4 pages below the BIOS ROM.
        bios_base - (4 * page_size)
    }

    /// Identity maps the required pages for the BIOS and the EPT.
    fn set_identity_mapping(&self) -> Result<()> {
        let mut base = self.ept_address();

        unsafe { kvm::ioctl::set_identity_map_addr(self.fd(), &mut base)? };

        Ok(())
    }

    /// Allocates a 3-page region in the first 4-GiBs of the guest address space.
    ///
    /// This must not conflict with any other memory slot.
    ///
    /// This is required on Intel-based hosts.
    fn set_tss_address(&self) -> Result<()> {
        let address = self.ept_address() + 0x1000;

        let address = address as i32;

        unsafe { kvm::ioctl::set_tss_addr(self.fd(), address)? };

        Ok(())
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

    fn allocate_memory(&self, memory: accel::MemoryRegion) -> Result<()> {
        use kvm::structs::mem;

        let mut region = mem::MemoryRegion::default();

        region.slot = memory.slot as u16;
        region.address_space = 0;

        region.host_virt_addr = memory.host.as_ptr() as u64;
        region.guest_phys_addr = memory.guest as u64;

        region.size = memory.host.len() as u64;

        unsafe { kvm::ioctl::set_memory_region(self.fd(), &mut region)? };

        Ok(())
    }

    fn create_vcpu<'b>(
        &'b self,
        slot: usize,
        cb: &'b accel::CpuCallbacks,
    ) -> Result<Box<accel::VirtualCPU<'b> + 'b>> {
        let slot = slot as i32;

        let fd = unsafe { kvm::ioctl::create_vcpu(self.fd(), slot)? };

        use std::os::unix::io::FromRawFd;
        let file = unsafe { File::from_raw_fd(fd as i32) };

        let vcpu = VirtualCPU::new(self, file, cb)?;

        Ok(Box::new(vcpu))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
