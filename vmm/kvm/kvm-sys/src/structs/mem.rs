//! Structures related to memory management.

/// A guest physical memory slot.
///
/// Memory regions in the same address space must not overlap.
///
/// The physical and virtual addresses should be aligned on 2 MiB boundaries,
/// for maximum efficiency.
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct MemoryRegion {
    /// The memory slot to be added or changed.
    ///
    /// Must be less than the `MaxMemSlots` capability.
    pub slot: u16,
    /// The address space of this region,
    /// if multiple address spaces are supported.
    ///
    /// The maximum number of address spaces is returned
    /// by the capability check.
    pub addr_space: u16,

    /// Flags describing the properties of this region.
    pub flags: Flags,

    // Reserved, used internally by KVM.
    _padding: u16,

    /// The address in the guest's physical address space.
    pub guest_phys_addr: u64,
    /// Size in bytes of this memory region.
    ///
    /// If `Flags::READ_ONLY` is set, this must be 0.
    pub memory_size: u64,
    /// Starting address of the host virtual memory region.
    pub host_virt_addr: u64,
}

bitflags! {
    #[derive(Default)]
    pub struct Flags: u16 {
        /// If set, KVM will keep track of writes to this slot.
        const LOG_DIRTY_PAGES = 1 << 0;
        /// This slot will be made read-only,
        /// any writes will notify sent to userspace.
        const READ_ONLY = 1 << 1;
    }
}
