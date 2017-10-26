/// Capabilities are additional features that the target machine might support.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Capability {
    /// Support for in-kernel IRQ chips.
    IrqChip = 0,
    /// Support for allocating memory from user space.
    ///
    /// This cap is always required since there is no other
    /// (non-obsolete) way of allocating memory.
    UserMemory = 3,
    /// Soft vCPU limit.
    MaxRecommendedVCpus = 9,
    /// Hard vCPU limit.
    MaxVCpus = 66,
    /// Support for ROM regions.
    ReadOnlyMemory = 81,
    EmulateCpuid = 95,
    /// Support for checking capabilities on VMs.
    /// Required for most other capabilities.
    CheckExtensionVM = 105,
    /// Maximum ID for virtual CPUs.
    MaxVCpuId = 128,
}
