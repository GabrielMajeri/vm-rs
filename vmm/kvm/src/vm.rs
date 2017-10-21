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

    /// Adds or modifies a block of memory.
    ///
    /// When changing an existing memory slot,
    /// only the guest physical address and its flags may be modified.
    ///
    /// Slots in each address space must not overlap.
    ///
    /// If you pass `None` for the host memory block, then this slot
    /// will be marked as read-only, and will be used for MMIO.
    pub fn set_memory_region(&self, memory: Option<usize>, size: usize, guest_phys_addr: u64, slot: u16, memory_space: u16) -> Result<()> {
        bitflags! {
            struct Flags: u32 {
                const LOG_DIRTY = 1;
                const READ_ONLY = 2;
            }
        }

        let mut flags = Flags::empty();

        let userspace_addr = {
            if let Some(memory) = memory {
                assert_eq!(memory as usize % 4096, 0, "Memory must be page-aligned");
                assert_eq!(size % 4096, 0, "Memory length must be a multiple of page size");
                memory as u64
            } else {
                flags |= Flags::READ_ONLY;
                0
            }
        };

        #[repr(C)]
        pub struct MemoryRegion {
            // TODO: query KVM_CAP_NR_MEMSLOTS.
            slot: u16,
            memory_space: u16,
            flags: Flags,
            guest_phys_addr: u64,
            memory_size: u64,
            userspace_addr: u64
        }

        let memory_region = MemoryRegion {
            slot,
            // TODO: support multiple memory spaces through KVM_CAP_MULTI_ADDRESS_SPACE.
            memory_space,
            flags,
            guest_phys_addr,
            memory_size: size as u64,
            userspace_addr,
        };

        ioctl!(write_ptr set_user_memory_region with KVM_IO, 0x46; MemoryRegion);

        unsafe {
            set_user_memory_region(self.fd, &memory_region)?;
        }

        Ok(())
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
