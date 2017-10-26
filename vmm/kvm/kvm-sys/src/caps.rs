/// Capabilities are additional features that the target machine might support.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Capability {
    UserMemory = 3,
    MaxVCPUs = 9,
    ReadOnlyMemory = 81,
    EmulateCpuid = 95,
    CheckExtensionVM = 105,
}
