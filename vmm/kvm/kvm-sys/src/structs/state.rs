//! Structures representing the virtual CPU's state.

#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct Registers {
    /// RAX, RBX, RCX, RDX,
    /// RSI, RDI, RSP, RBP
    /// then R8 through R15.
    pub r: [u64; 16],
    /// Instruction pointer.
    pub ip: u64,
    /// FLAGS register.
    pub flags: u64,
}
