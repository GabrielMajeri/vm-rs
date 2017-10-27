//! Structures representing the x86 processor state.

use fpu;

/// Stores information about a memory segment.
#[derive(Debug, Copy, Clone)]
pub struct Segment {
    /// The starting physical address of this segment,
    pub base: u64,
    /// The size of the segment. [base, base + limit) must be a valid memory range.
    pub limit: u32,
    /// The selector of this segment. Equal to the index in the GDT.
    pub selector: u16,
    /// Bits:
    /// - 0: Accessed
    /// - 1: Writable (for data) / Readable (for code)
    /// - 2: Direction (for data) / Conforming (for code)
    /// - 3: True for code, false for data.
    /// - 4: True for user descriptors, false for system descriptors.
    /// - 5-6: Descriptor Privilege Level.
    /// - 7: Is present.
    /// - 8-11: Limit (unused).
    /// - 12: available for OS to use.
    /// - 13: Is in long mode.
    /// - 14: Operand size (16 bit / 32 bit).
    /// - 15: Granularity (byte / 4-KiB).
    pub flags: u16,
}

/// Stores the state for an `x86_64` CPU.
#[derive(Debug, Copy, Clone)]
pub struct State {
    /// This is a short list of the general-purpose registers.
    ///
    /// Some of these "general-purpose" register actually have a use in various function calls.
    /// Format: R# = <name> (<mnemonic>)
    /// - R0 = Accumulator (RAX)
    /// - R1 = Counter (RCX)
    /// - R2 = Data (RDX)
    /// - R3 = Base (RBX)
    /// - R4 = Stack Pointer (SP)
    /// - R5 = Base Pointer (BP)
    /// - R6 = Source Index (SI)
    /// - R7 = Destination Index (DI)
    /// - R8 through R15 - General-purpose
    pub r: [u64; 16],
    /// The instruction pointer stores the address of the current instruction.
    pub ip: u64,
    /// The FLAGS register.
    pub flags: Flags,
    /// The current code segment, a segment of memory which contains executable code.
    pub cs: Segment,
    /// The current data segment.
    pub ds: Segment,
    /// The stack segment.
    pub ss: Segment,
    /// Extra segment #1.
    pub es: Segment,
    /// Extra segment #2.
    pub fs: Segment,
    /// Extra segment #3.
    pub gs: Segment,
    /// Control register 0.
    pub cr0: u64,

    /// The x87 floating-point unit's state.
    pub fpu: fpu::X87State,
    /// The SSE / AVX registers.
    pub sse: fpu::SseState,
}

impl Default for State {
    fn default() -> Self {
        // The `default` state is the state of the processor after startup.
        // See Intel Architecture Manual, Vol. 3A, Section "9.1.1 Processor State After Reset"

        let mut r = [0; 16];

        // TODO: this is supposed to be 0xn0600, where n is extended model value (what is that?).
        // For now, it is 1.
        r[2] = 0x1_0600;

        // We must start at 16 bytes before 4 GiB.
        let ip = 0xFFF0;
        let cs = Segment {
            base: 0xFFFF_0000,
            selector: 0xF000,
            limit: 0xFFFF,
            // Accessed, readable, user, present.
            flags: 0b1001_0011,
        };

        let ds = Segment {
            base: 0,
            selector: 0,
            limit: 0xFFFF,
            // Accessed, readable, user, present.
            flags: 0b1001_0011,
        };

        State {
            r,
            ip,
            cs,
            ds,
            ss: ds,
            es: ds,
            fs: ds,
            gs: ds,
            flags: Flags::default(),
            // The CD and NW flags and bit 4 are set.
            cr0: (1 << 30) | (1 << 29) | (1 << 4),
            fpu: fpu::X87State::default(),
            sse: fpu::SseState::default(),
        }
    }
}

bitflags! {
    /// The FLAGS register contains flags set by various operations,
    /// as well as some useful flags which change the behaviour of
    /// basic operations.
    ///
    /// All other bits are reserved and must be preserved.
    pub struct Flags: u64 {
        /// Carry flag, set if last operation resulted in a carry.
        const CARRY = 1 << 0;

        /// Reserved, must always be set.
        const RESERVED_ONE = 1 << 1;

        /// Parity of last result.
        const PARITY = 1 << 2;

        /// Adjust flag. Used for binary-coded decimals.
        const ADJUST = 1 << 4;

        /// Last value was 0.
        const ZERO = 1 << 6;

        /// Sign of last result.
        const SIGN = 1 << 7;

        /// If set, stops execution after every instruction,
        /// for debugging purposes.
        const TRAP = 1 << 8;

        /// Enables or disables extern interrupts.
        ///
        /// Note that exceptions are NMIs are not affected.
        const INTERRUPT = 1 << 9;

        /// If set, direction of `rep` instructions is backwards.
        const DIRECTION = 1 << 10;

        /// Set if an operation resulted in an overflow.
        const OVERFLOW = 1 << 11;

        /// Privilege level required for port I/O operations.
        const IOPRIV = 0b11 << 12;

        /// Indicates that a system task invoked another
        /// using a CALL instruction.
        const NESTED_TASK = 1 << 14;

        /// Used to temporarily disable debug exceptions.
        const RESUME = 1 << 16;

        /// Virtual 8086 mode.
        const VIRTUAL_8086 = 1 << 17;

        /// When set, all memory accesses must be aligned.
        const ALIGNMENT_CHECK = 1 << 18;

        /// Enables virtual interrupts.
        const VIRTUAL_INTERRUPT = 1 << 19;

        /// A virtual interrupt is pending.
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;

        /// CPU supports CPUID instruction.
        const CPUID = 1 << 21;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::RESERVED_ONE
    }
}
