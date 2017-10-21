/// Extensions are additional APIs that the target machine might support.
///
/// Since Linux 2.6.22, the KVM ABI is stable, meaning new changes may
/// only be added as extensions queried at run time.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Extension {
    UserMemory = 3,
    MaxVCPUs = 9,
    ReadOnlyMemory = 81,
}
