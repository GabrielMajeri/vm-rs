//! Support for model-specific registers.

/// Time-stamp counter.
pub const TSC: u32 = 0x10;

/// SYSENTER code segment.
pub const SYSENTER_CS: u32 = 0x174;
/// SYSENTER stack pointer.
pub const SYSENTER_SP: u32 = 0x175;
/// SYSENTER instruction pointer.
pub const SYSENTER_IP: u32 = 0x176;

/// Contains all the architectural MSRs.
#[derive(Debug, Copy, Clone)]
pub struct MSRState {
    /// The time-stamp counter.
    pub tsc: u64,
    /// The SYSENTER code segment.
    pub sysenter_cs: u64,
}
