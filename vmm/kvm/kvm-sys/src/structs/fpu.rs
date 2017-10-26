#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FpuState {
    pub regs: [[u64; 2]; 8],
    pub control_word: u16,
    pub status_word: u16,
    /// In `FXSTOR` single-byte format.
    pub tag_word: u8,
    _padding1: u8,
    pub last_opcode: u16,
    /// Last instruction pointer.
    pub last_ip: u64,
    /// Last data pointer.
    pub last_dp: u64,
    /// Contents of SSE / AVX registers.
    pub xmm: [[u64; 2]; 16],
    /// SSE control / status register.
    pub mxcsr: u32,
    _padding2: u32,
}
